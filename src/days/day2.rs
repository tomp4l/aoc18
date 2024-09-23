use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let part1 = checksum(input).to_string();
        let part2 = common_letters(input).ok_or("No common letters found")?;
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn checksum(ids: &str) -> i32 {
    let (a, b) = ids.lines().fold((0, 0), |(a, b), s| {
        let counts = s.chars().counts();
        (
            a + (counts.values().contains(&2) as i32),
            b + (counts.values().contains(&3) as i32),
        )
    });
    a * b
}

fn common_letters(ids: &str) -> Option<String> {
    ids.lines()
        .tuple_combinations::<(_, _)>()
        .find_map(|(a, b)| {
            let common: String = a
                .chars()
                .zip(b.chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect();
            if common.len() == a.len() - 1 {
                Some(common)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_example() {
        let example = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        assert_eq!(checksum(example), 12);
    }

    #[test]
    fn example() {
        let instance = Instance;
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        let expected = DayResult {
            part1: "0".to_owned(),
            part2: Some("fgij".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
