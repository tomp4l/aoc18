use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let part1 = fully_reduce_polymer(input).len().to_string();
        let part2 = (b'A'..=b'Z')
            .map(|unit| {
                let unit = unit as char;
                let reduced = fully_reduce_polymer(&remove_unit(input, unit));
                reduced.len()
            })
            .min()
            .map(|len| len.to_string());
        Ok(DayResult { part1, part2 })
    }
}

fn reduce_polymer(polymer: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    for c in polymer.chars() {
        if let Some(&last) = stack.last() {
            if last != c && last.eq_ignore_ascii_case(&c) {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
    }
    stack.iter().collect()
}

fn remove_unit(polymer: &str, unit: char) -> String {
    polymer
        .chars()
        .filter(|&c| !c.eq_ignore_ascii_case(&unit))
        .collect()
}

fn fully_reduce_polymer(polymer: &str) -> String {
    let mut polymer = polymer.to_owned();
    loop {
        let reduced = reduce_polymer(&polymer);
        if reduced == polymer {
            return reduced;
        }
        polymer = reduced;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "dabAcCaCBAcCcaDA";
        let expected = DayResult {
            part1: "10".to_owned(),
            part2: Some("4".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
