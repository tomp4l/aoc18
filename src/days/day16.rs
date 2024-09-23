use std::collections::HashSet;

use itertools::Itertools;

use super::day::*;
use super::instructions::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let samples = parse_samples(input);
        let part1 = part1(&samples).to_string();

        let program = parse_program(input);

        let part2 = part2(&samples, &program).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct UnknownOpcode {
    opcode: i32,
    a: i64,
    b: i64,
    c: i64,
}

impl UnknownOpcode {
    fn new(opcode: i32, a: i64, b: i64, c: i64) -> Self {
        Self { opcode, a, b, c }
    }

    fn behaves_like(
        &self,
        registers_before: &Registers,
        registers_after: &Registers,
    ) -> Vec<&'static Opcode> {
        Opcode::iter()
            .filter(|opcode| {
                &opcode.apply(self.a, self.b, self.c, registers_before) == registers_after
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sample {
    before: Registers,
    instruction: UnknownOpcode,
    after: Registers,
}

impl Sample {
    fn new(before: Registers, instruction: UnknownOpcode, after: Registers) -> Self {
        Self {
            before,
            instruction,
            after,
        }
    }
}

fn parse_samples(input: &str) -> Vec<Sample> {
    input
        .split("\n\n\n\n")
        .next()
        .unwrap()
        .lines()
        .chunks(4)
        .into_iter()
        .filter_map(|mut lines| {
            let parts: Vec<_> = lines
                .join(" ")
                .split_whitespace()
                .map(|s| s.trim_matches(|c| c == '[' || c == ']' || c == ','))
                .filter_map(|part| part.parse::<i32>().ok())
                .collect();
            if parts.len() == 12 {
                Some(Sample::new(
                    Registers::from_slice(&[
                        parts[0].into(),
                        parts[1].into(),
                        parts[2].into(),
                        parts[3].into(),
                    ]),
                    UnknownOpcode::new(
                        parts[4].into(),
                        parts[5].into(),
                        parts[6].into(),
                        parts[7].into(),
                    ),
                    Registers::from_slice(&[
                        parts[8].into(),
                        parts[9].into(),
                        parts[10].into(),
                        parts[11].into(),
                    ]),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn parse_program(input: &str) -> Vec<UnknownOpcode> {
    input
        .split("\n\n\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split_whitespace()
                .filter_map(|part| part.parse::<i32>().ok())
                .collect();
            UnknownOpcode::new(parts[0], parts[1].into(), parts[2].into(), parts[3].into())
        })
        .collect()
}

fn part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .filter(|sample| {
            let opcode = &sample.instruction;
            let opcodes = opcode.behaves_like(&sample.before, &sample.after);
            opcodes.len() >= 3
        })
        .count()
}

fn part2(samples: &[Sample], program: &[UnknownOpcode]) -> i64 {
    let mut opcode_map: Vec<HashSet<Opcode>> = vec![HashSet::new(); 16];

    for sample in samples {
        let opcode = &sample.instruction;
        let opcodes = opcode.behaves_like(&sample.before, &sample.after);

        let current_opcodes = &mut opcode_map[opcode.opcode as usize];
        if current_opcodes.is_empty() {
            *current_opcodes = opcodes.into_iter().cloned().collect();
        } else {
            current_opcodes.retain(|opcode| opcodes.contains(&opcode));
        }
    }

    while opcode_map.iter().any(|opcodes| opcodes.len() > 1) {
        let known_opcodes: HashSet<Opcode> = opcode_map
            .iter()
            .filter_map(|opcodes| {
                if opcodes.len() == 1 {
                    Some(opcodes.iter().next().unwrap().clone())
                } else {
                    None
                }
            })
            .collect();

        for opcodes in opcode_map.iter_mut() {
            if opcodes.len() > 1 {
                *opcodes = opcodes.difference(&known_opcodes).cloned().collect();
            }
        }
    }

    let mut registers = Registers::new(4);
    for instruction in program {
        registers = opcode_map[instruction.opcode as usize]
            .iter()
            .next()
            .unwrap()
            .apply(instruction.a, instruction.b, instruction.c, &registers);
    }

    registers.get(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"#;
        let samples = parse_samples(input);
        assert_eq!(samples.len(), 1);
        let sample = &samples[0];
        assert_eq!(
            sample,
            &Sample::new(
                Registers::from_slice(&[3, 2, 1, 1]),
                UnknownOpcode::new(9, 2, 1, 2),
                Registers::from_slice(&[3, 2, 2, 1])
            )
        );

        assert_eq!(
            sample
                .instruction
                .behaves_like(&sample.before, &sample.after)
                .len(),
            3
        );
    }
}
