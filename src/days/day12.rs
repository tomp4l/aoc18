use std::str::FromStr;

use itertools::{FoldWhile, Itertools};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut pot_rules = input.parse::<PotRules>()?;

        let part1 = pot_rules.nth(19).unwrap().to_string();

        let (num, _, diff, start) = pot_rules
            .zip(21..)
            .fold_while(
                (0, 0, 0, 0),
                |(_, count, last_diff, last_value), (value, i)| {
                    let diff = value - last_value;
                    if diff == last_diff {
                        if count < 100 {
                            FoldWhile::Continue((i, count + 1, diff, value))
                        } else {
                            FoldWhile::Done((i, count, diff, value))
                        }
                    } else {
                        FoldWhile::Continue((i, 0, diff, value))
                    }
                },
            )
            .into_inner();

        let part2 = start as i64 + (50_000_000_000 - num) * diff as i64;

        Ok(DayResult {
            part1,
            part2: Some(part2.to_string()),
        })
    }
}

struct Pots {
    pots: Vec<bool>,
    offset: i32,
}

impl FromStr for Pots {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let initial_state = s
            .split_whitespace()
            .nth(2)
            .ok_or("missing initial state")?
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();
        let offset = -4;
        let mut pots = vec![false; 4];
        pots.extend(initial_state);
        pots.extend(vec![false; 4]);
        Ok(Pots { pots, offset })
    }
}

impl Pots {
    fn next_generation(&mut self, rules: &[(Vec<bool>, bool)]) {
        let mut next_pots = vec![false; self.pots.len() + 4];
        for i in 2..self.pots.len() - 2 {
            let pots = &self.pots[i - 2..=i + 2];
            for (rule, result) in rules {
                if pots == rule {
                    next_pots[i + 2] = *result;
                    break;
                }
            }
        }

        let start = next_pots
            .windows(5)
            .take_while(|a| a.iter().all(|t| !*t))
            .count();
        let end = next_pots
            .windows(5)
            .rev()
            .take_while(|a| a.iter().all(|t| !*t))
            .count();

        self.pots = next_pots[start..next_pots.len() - end].to_vec();
        self.offset += start as i32 - 2;
    }

    fn alive_pots(&self) -> i32 {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, &alive)| alive)
            .map(|(i, _)| i as i32 + self.offset)
            .sum()
    }
}

fn parse_rule(s: &str) -> Result<(Vec<bool>, bool), String> {
    let parts = s.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err(format!("invalid rule: {}", s));
    }
    let rule = parts[0].chars().map(|c| c == '#').collect::<Vec<_>>();
    let result = parts[2] == "#";
    Ok((rule, result))
}

impl FromStr for PotRules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let pots = lines.next().unwrap().parse::<Pots>().unwrap();
        let rules = lines
            .skip(1)
            .map(|line| parse_rule(line).unwrap())
            .collect::<Vec<_>>();
        Ok(PotRules { pots, rules })
    }
}

struct PotRules {
    pots: Pots,
    rules: Vec<(Vec<bool>, bool)>,
}

impl Iterator for PotRules {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.pots.next_generation(&self.rules);
        Some(self.pots.alive_pots())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

        let expected = DayResult {
            part1: "325".to_owned(),
            part2: Some("999999999374".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
