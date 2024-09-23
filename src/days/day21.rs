use std::collections::HashSet;

use super::{day::*, instructions::Cpu};

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let part1 = part1(input.parse::<Cpu>()?);
        let part2 = part2(input.parse::<Cpu>()?);

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn part1(mut cpu: Cpu) -> String {
    while cpu.step() {
        let ip = cpu.ip();
        if ip == 29 {
            return cpu.get(4).to_string();
        }
    }

    unreachable!()
}

fn part2(mut cpu: Cpu) -> String {
    let mut seen = HashSet::new();
    let mut last = 0;
    while cpu.step() {
        let ip = cpu.ip();

        // Skip running the slow loop and precompute the result
        if ip == 18 {
            let r3 = cpu.get(3);
            let t = r3 / 256;
            cpu.set(2, t);
            cpu.set(5, (t + 1) * 256);
            cpu.set(1, 20);
        }

        if ip == 29 {
            if seen.contains(&cpu.get(4)) {
                return last.to_string();
            }
            last = cpu.get(4);
            seen.insert(last);
        }
    }

    unreachable!()
}
