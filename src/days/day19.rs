use super::{day::*, instructions::Cpu};

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut cpu = input.parse::<Cpu>()?;
        cpu.run();
        let part1 = cpu.get(0).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2(input).to_string()),
        })
    }
}

fn sum_of_divisors(n: i64) -> i64 {
    (1..=(n / 2)).filter(|i| n % i == 0).sum::<i64>() + n
}

fn part2(input: &str) -> i64 {
    let mut cpu = input.parse::<Cpu>().unwrap();
    cpu.set(0, 1);

    let mut prev = 0;
    let mut count = 0;
    loop {
        cpu.step();
        let r2 = cpu.get(2);
        if r2 != prev {
            count = 0;
            prev = r2;
        }
        count += 1;
        if count == 100 {
            break;
        }
    }

    sum_of_divisors(cpu.get(2))
}
