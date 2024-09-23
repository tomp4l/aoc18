use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut area = input.parse::<Area>()?;
        let part1 = area.risk_level().to_string();
        let part2 = Some(area.traverse().to_string());
        Ok(DayResult { part1, part2 })
    }
}

struct Area {
    depth: i32,
    target: (i32, i32),
    geologic_indices: HashMap<(i32, i32), i32>,
}

impl FromStr for Area {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .lines()
            .map(|line| {
                line.split([' ', ','].as_ref())
                    .filter_map(|part| part.parse::<i32>().ok())
                    .collect_vec()
            })
            .collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("invalid area: {}", s));
        }
        if let (Some(&depth), Some(&x), Some(&y)) =
            (parts[0].get(0), parts[1].get(0), parts[1].get(1))
        {
            return Ok(Area {
                depth,
                target: (x, y),
                geologic_indices: HashMap::new(),
            });
        } else {
            return Err(format!("invalid area: {}", s));
        }
    }
}

impl Area {
    fn geologic_index(&mut self, x: i32, y: i32) -> i32 {
        let index = if (x, y) == (0, 0) || (x, y) == self.target {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else if let Some(v) = self.geologic_indices.get(&(x, y)) {
            *v
        } else {
            let v = self.geologic_index(x - 1, y) * self.geologic_index(x, y - 1);

            self.geologic_indices.insert((x, y), v % 20183);

            v
        };

        (index + self.depth) % 20183
    }

    fn erosion_level(&mut self, x: i32, y: i32) -> i32 {
        self.geologic_index(x, y) % 3
    }

    fn risk_level(&mut self) -> i32 {
        (0..=self.target.0)
            .cartesian_product(0..=self.target.1)
            .map(|(x, y)| self.erosion_level(x, y))
            .sum()
    }

    fn traverse(&mut self) -> i32 {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct State {
            position: (i32, i32),
            tool: Tool,
        }

        let mut visited = HashMap::new();
        let mut to_visit = HashMap::new();
        to_visit.insert(
            0,
            vec![State {
                position: (0, 0),
                tool: Tool::Torch,
            }],
        );

        fn process_state(
            state: State,
            minutes: i32,
            visited: &mut HashMap<State, i32>,
            to_visit: &mut HashMap<i32, Vec<State>>,
            area: &mut Area,
        ) -> Option<i32> {
            if state.position == area.target && state.tool == Tool::Torch {
                return Some(minutes);
            }

            if let Some(&visited_minutes) = visited.get(&state) {
                if visited_minutes <= minutes {
                    return None;
                }
            }

            visited.insert(state.clone(), minutes);

            let (x, y) = state.position;
            let current_erosion = area.erosion_level(x, y);

            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_position = (x + dx, y + dy);
                if new_position.0 < 0 || new_position.1 < 0 {
                    continue;
                }
                let new_erosion = area.erosion_level(new_position.0, new_position.1);

                add_new_states(
                    current_erosion,
                    new_erosion,
                    state.tool,
                    new_position,
                    minutes,
                    visited,
                    to_visit,
                );
            }
            return None;
        }

        fn add_new_states(
            current_erosion: i32,
            new_erosion: i32,
            current_tool: Tool,
            new_position: (i32, i32),
            minutes: i32,
            visited: &HashMap<State, i32>,
            to_visit: &mut HashMap<i32, Vec<State>>,
        ) {
            for &tool in &[Tool::Torch, Tool::ClimbingGear, Tool::Neither] {
                if !tool.is_valid(current_erosion) {
                    continue;
                }
                if !tool.is_valid(new_erosion) {
                    continue;
                }

                let minutes = minutes + if current_tool == tool { 1 } else { 8 };
                if let Some(&visited_minutes) = visited.get(&State {
                    position: new_position,
                    tool,
                }) {
                    if visited_minutes <= minutes {
                        continue;
                    }
                }
                to_visit
                    .entry(minutes)
                    .and_modify(|state| {
                        state.push(State {
                            position: new_position,
                            tool,
                        })
                    })
                    .or_insert_with(|| {
                        vec![State {
                            position: new_position,
                            tool,
                        }]
                    });
            }
        }

        while !to_visit.is_empty() {
            let (minutes, states) = to_visit
                .iter()
                .min_by_key(|(minutes, _)| minutes.to_owned())
                .unwrap();

            let states = states.to_owned();
            let minutes = *minutes;
            to_visit.remove(&minutes);

            for state in states {
                if let Some(m) = process_state(state, minutes, &mut visited, &mut to_visit, self) {
                    return m;
                }
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn is_valid(&self, erosion: i32) -> bool {
        match self {
            Tool::Torch => erosion != 1,
            Tool::ClimbingGear => erosion != 2,
            Tool::Neither => erosion != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "depth: 510
target: 10,10
";
        let expected = DayResult {
            part1: "114".to_owned(),
            part2: Some("45".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
