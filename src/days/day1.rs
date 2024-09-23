use std::collections::HashSet;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let numbers = parse(input)?;
        let part1 = numbers.iter().sum::<i32>().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2(&numbers).to_string()),
        })
    }
}

fn parse(input: &str) -> Result<Vec<i32>, String> {
    input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())
}

fn part2(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .cycle()
        .scan((0, HashSet::new()), |state, &x| {
            state.0 += x;
            let c = state.1.contains(&state.0);
            state.1.insert(state.0);

            Some((state.0, c))
        })
        .find_map(|x| if x.1 { Some(x.0) } else { None })
        .expect("infinitely repeating sequence")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "+1
-2
+3
+1";
        let expected = DayResult {
            part1: "3".to_owned(),
            part2: Some("2".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
