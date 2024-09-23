use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let steps = input.parse::<usize>().map_err(|e| e.to_string())?;

        let part1 = run_for_steps(steps);
        let part2 = run_for_end(steps).to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

struct Recipes {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl Recipes {
    fn new() -> Self {
        Self {
            recipes: vec![3, 7],
            elf1: 0,
            elf2: 1,
        }
    }

    fn step(&mut self) {
        let sum = self.recipes[self.elf1] + self.recipes[self.elf2];
        if sum >= 10 {
            self.recipes.push(sum / 10);
        }
        self.recipes.push(sum % 10);

        self.elf1 = (self.elf1 + 1 + self.recipes[self.elf1] as usize) % self.recipes.len();
        self.elf2 = (self.elf2 + 1 + self.recipes[self.elf2] as usize) % self.recipes.len();
    }
}

fn run_for_steps(steps: usize) -> String {
    let mut recipes = Recipes::new();
    while recipes.recipes.len() < steps + 10 {
        recipes.step();
    }
    recipes.recipes[steps..steps + 10]
        .iter()
        .map(|&n| (n + b'0') as char)
        .collect()
}

fn run_for_end(end: usize) -> usize {
    let mut recipes = Recipes::new();

    let end = end.to_string();
    let end = end
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    loop {
        recipes.step();
        if recipes.recipes.len() >= end.len() {
            if recipes.recipes[recipes.recipes.len() - end.len()..] == end[..] {
                return recipes.recipes.len() - end.len();
            }
            if recipes.recipes.len() > end.len() {
                if recipes.recipes[recipes.recipes.len() - end.len() - 1..recipes.recipes.len() - 1]
                    == end[..]
                {
                    return recipes.recipes.len() - end.len() - 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "9";
        let expected = DayResult {
            part1: "5158916779".to_owned(),
            part2: Some("13".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
