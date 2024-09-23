use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance {
    additional_time: usize,
    max_workers: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            additional_time: 60,
            max_workers: 5,
        }
    }
}

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let steps = input.parse::<Steps>()?;
        let part1 = steps.order();
        let part2 = steps.time(self.additional_time, self.max_workers);
        Ok(DayResult {
            part1,
            part2: Some(part2.to_string()),
        })
    }
}

struct Steps {
    step_depends_on: HashMap<char, Vec<char>>,
}

impl FromStr for Steps {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut steps = HashMap::new();
        for line in s.lines() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts.len() != 10 {
                return Err(format!("invalid line: {}", parts.len()));
            }
            let step = parts[1].chars().next().ok_or_else(|| "missing step")?;
            let depends_on = parts[7]
                .chars()
                .next()
                .ok_or_else(|| "missing depends on")?;
            steps.entry(step).or_insert_with(Vec::new);
            steps.entry(depends_on).or_insert_with(Vec::new).push(step);
        }
        Ok(Steps {
            step_depends_on: steps,
        })
    }
}

fn next_step(steps: &HashMap<char, Vec<char>>) -> Option<char> {
    let mut next = None;
    for (step, depends_on) in steps {
        if depends_on.is_empty() {
            if let Some(n) = next {
                if *step < n {
                    next = Some(*step);
                }
            } else {
                next = Some(*step);
            }
        }
    }
    next
}

impl Steps {
    fn order(&self) -> String {
        let mut steps = self.step_depends_on.clone();
        let mut order = String::new();
        while !steps.is_empty() {
            let next = next_step(&steps);
            if let Some(next) = next {
                order.push(next);
                steps.remove(&next);
                for depends_on in steps.values_mut() {
                    depends_on.retain(|&step| step != next);
                }
            }
        }
        order
    }

    fn time(&self, additional_time: usize, max_workers: usize) -> usize {
        let mut steps = self.step_depends_on.clone();
        let mut workers = vec![None; max_workers];
        let mut time = 0;
        while !steps.is_empty() {
            for worker in &mut workers {
                if worker.is_none() {
                    let next = next_step(&steps);
                    if let Some(next) = next {
                        *worker = Some((next, next as usize - b'A' as usize + 1 + additional_time));
                        steps.remove(&next);
                    }
                }
            }
            let min_time = workers
                .iter()
                .filter_map(|w| w.as_ref().map(|(_, t)| t))
                .min()
                .copied();
            if let Some(min_time) = min_time {
                time += min_time;
                for worker in &mut workers {
                    if let Some((step, t)) = worker {
                        *t -= min_time;
                        if *t == 0 {
                            for depends_on in steps.values_mut() {
                                depends_on.retain(|&s| s != *step);
                            }
                            *worker = None;
                        }
                    }
                }
            }
        }
        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance {
            additional_time: 0,
            max_workers: 2,
        };
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let expected = DayResult {
            part1: "CABDFE".to_owned(),
            part2: Some("15".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
