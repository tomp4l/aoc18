use std::{slice::Iter, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Registers(Vec<i64>);

impl Registers {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }

    pub fn from_slice(slice: &[i64]) -> Self {
        Self(slice.to_vec())
    }

    pub fn get(&self, index: usize) -> i64 {
        self.0[index]
    }

    pub fn set(&mut self, index: usize, value: i64) {
        self.0[index] = value;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    pub fn iter() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode; 16] = [
            Opcode::Addr,
            Opcode::Addi,
            Opcode::Mulr,
            Opcode::Muli,
            Opcode::Banr,
            Opcode::Bani,
            Opcode::Borr,
            Opcode::Bori,
            Opcode::Setr,
            Opcode::Seti,
            Opcode::Gtir,
            Opcode::Gtri,
            Opcode::Gtrr,
            Opcode::Eqir,
            Opcode::Eqri,
            Opcode::Eqrr,
        ];
        OPCODES.iter()
    }

    pub fn apply(&self, a: i64, b: i64, c: i64, registers: &Registers) -> Registers {
        let mut registers = registers.clone();
        match self {
            Opcode::Addr => {
                registers.0[c as usize] = registers.0[a as usize] + registers.0[b as usize]
            }
            Opcode::Addi => registers.0[c as usize] = registers.0[a as usize] + b,
            Opcode::Mulr => {
                registers.0[c as usize] = registers.0[a as usize] * registers.0[b as usize]
            }
            Opcode::Muli => registers.0[c as usize] = registers.0[a as usize] * b,
            Opcode::Banr => {
                registers.0[c as usize] = registers.0[a as usize] & registers.0[b as usize]
            }
            Opcode::Bani => registers.0[c as usize] = registers.0[a as usize] & b,
            Opcode::Borr => {
                registers.0[c as usize] = registers.0[a as usize] | registers.0[b as usize]
            }
            Opcode::Bori => registers.0[c as usize] = registers.0[a as usize] | b,
            Opcode::Setr => registers.0[c as usize] = registers.0[a as usize],
            Opcode::Seti => registers.0[c as usize] = a,
            Opcode::Gtir => {
                registers.0[c as usize] = if a > registers.0[b as usize] { 1 } else { 0 }
            }
            Opcode::Gtri => {
                registers.0[c as usize] = if registers.0[a as usize] > b { 1 } else { 0 }
            }
            Opcode::Gtrr => {
                registers.0[c as usize] = if registers.0[a as usize] > registers.0[b as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqir => {
                registers.0[c as usize] = if a == registers.0[b as usize] { 1 } else { 0 }
            }
            Opcode::Eqri => {
                registers.0[c as usize] = if registers.0[a as usize] == b { 1 } else { 0 }
            }
            Opcode::Eqrr => {
                registers.0[c as usize] = if registers.0[a as usize] == registers.0[b as usize] {
                    1
                } else {
                    0
                }
            }
        }
        registers
    }
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Opcode::Addr),
            "addi" => Ok(Opcode::Addi),
            "mulr" => Ok(Opcode::Mulr),
            "muli" => Ok(Opcode::Muli),
            "banr" => Ok(Opcode::Banr),
            "bani" => Ok(Opcode::Bani),
            "borr" => Ok(Opcode::Borr),
            "bori" => Ok(Opcode::Bori),
            "setr" => Ok(Opcode::Setr),
            "seti" => Ok(Opcode::Seti),
            "gtir" => Ok(Opcode::Gtir),
            "gtri" => Ok(Opcode::Gtri),
            "gtrr" => Ok(Opcode::Gtrr),
            "eqir" => Ok(Opcode::Eqir),
            "eqri" => Ok(Opcode::Eqri),
            "eqrr" => Ok(Opcode::Eqrr),
            _ => Err(format!("Invalid opcode: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    a: i64,
    b: i64,
    c: i64,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(format!("invalid instruction: {}", s));
        }
        let opcode = parts[0].parse()?;
        let a = parts[1]
            .parse::<i64>()
            .map_err(|e| format!("invalid a: {}", e))?;
        let b = parts[2]
            .parse::<i64>()
            .map_err(|e| format!("invalid b: {}", e))?;
        let c = parts[3]
            .parse::<i64>()
            .map_err(|e| format!("invalid c: {}", e))?;
        Ok(Instruction { opcode, a, b, c })
    }
}

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
    ip: usize,
    instructions: Vec<Instruction>,
}

impl FromStr for Cpu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let ip = lines
            .next()
            .ok_or("missing ip")?
            .split_whitespace()
            .nth(1)
            .ok_or("missing ip value")?
            .parse::<usize>()
            .map_err(|e| format!("invalid ip: {}", e))?;
        let instructions = lines
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Cpu {
            registers: Registers::new(6),
            ip,
            instructions,
        })
    }
}

impl Cpu {
    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        if let Some(instruction) = self.instructions.get(self.registers.get(self.ip) as usize) {
            self.registers = instruction.opcode.apply(
                instruction.a,
                instruction.b,
                instruction.c,
                &self.registers,
            );
            self.registers.set(self.ip, self.registers.get(self.ip) + 1);

            true
        } else {
            false
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        self.registers.get(index)
    }

    pub fn set(&mut self, index: usize, value: i64) {
        self.registers.set(index, value);
    }

    pub fn ip(&self) -> i64 {
        self.registers.get(self.ip)
    }

    #[allow(dead_code)]
    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let mut cpu = input.parse::<Cpu>().unwrap();
        cpu.run();
        assert_eq!(cpu.get(0), 7);
    }
}
