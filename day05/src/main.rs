use std::io;
use std::io::{BufRead, Error};
use Opcode::*;
use State::*;

type C = i32;
// contents
type P = usize;
// pointers
type Memory = Vec<C>;

#[derive(Debug, Copy, Clone)]
enum Mode {
    REFERENCE,
    VALUE,
}

#[derive(Debug, Clone)]
struct Instruction {
    code: C,
    modes: Vec<Mode>,
}

impl Instruction {
    fn from(value: C) -> Self {
        let s = format!("{:}", value);
        let code: i32 = s
            .chars()
            .rev()
            .take(2)
            .map(|c| c.to_digit(10).unwrap() as i32)
            .enumerate()
            .map(|(i, d)| 10_i32.pow(i as u32) * d)
            .sum();
        let modes: Vec<Mode> = s
            .chars()
            .rev()
            .skip(2)
            .map(|c| match c {
                '0' => Mode::REFERENCE,
                '1' => Mode::VALUE,
                _ => panic!("Invalid mode"),
            })
            .collect();
        Instruction { code, modes }
    }

    fn get_mode(&self, i: usize) -> Mode {
        if i < self.modes.len() {
            return self.modes[i];
        }
        Mode::REFERENCE
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Opcode {
    INIT,
    ADD(C, C, P),
    MUL(C, C, P),
    INPUT(P),
    OUTPUT(C),
    JMPT(C, P),
    JMPF(C, P),
    LT(C, C, P),
    EQ(C, C, P),
    EXIT,
}

#[derive(Debug, Clone)]
struct Step {
    opcode: Opcode,
    raw: Memory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    RUNNING,
    HALTED,
}

#[derive(Debug, Clone)]
struct Program {
    memory: Memory,
    eip: P,
    counter: usize,
    state: State,
    last: Step,
    inputs: Vec<C>,
    outputs: Vec<C>,
}

impl Program {
    fn new(memory: Memory) -> Self {
        Program {
            memory,
            eip: 0,
            counter: 0,
            state: RUNNING,
            last: Step {
                opcode: INIT,
                raw: Vec::new(),
            },
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn fetch(&self, value: C, mode: Mode) -> C {
        match mode {
            Mode::VALUE => value,
            Mode::REFERENCE => self.memory[value as P],
        }
    }

    fn fetch_offset(&self, offset: usize, mode: Mode) -> C {
        let value: C = self.memory[self.eip + offset];
        self.fetch(value, mode)
    }

    fn extract_opcode(&self) -> Opcode {
        let instr = Instruction::from(self.memory[self.eip]);
        match instr.code {
            1 => ADD(
                self.fetch_offset(1, instr.get_mode(0)),
                self.fetch_offset(2, instr.get_mode(1)),
                self.fetch_offset(3, Mode::VALUE) as P,
            ),
            2 => MUL(
                self.fetch_offset(1, instr.get_mode(0)),
                self.fetch_offset(2, instr.get_mode(1)),
                self.fetch_offset(3, Mode::VALUE) as P,
            ),
            3 => INPUT(self.fetch_offset(1, Mode::VALUE) as P),
            4 => OUTPUT(self.fetch_offset(1, instr.get_mode(0))),
            5 => JMPT(
                self.fetch_offset(1, instr.get_mode(0)),
                self.fetch_offset(2, instr.get_mode(1)) as P,
            ),
            6 => JMPF(
                self.fetch_offset(1, instr.get_mode(0)),
                self.fetch_offset(2, instr.get_mode(1)) as P,
            ),
            7 => LT(
                self.fetch_offset(1, instr.get_mode(0)),
                self.fetch_offset(2, instr.get_mode(1)),
                self.fetch_offset(3, Mode::VALUE) as P,
            ),
            8 => EQ(
                self.fetch_offset(1, instr.get_mode(0)),
                self.fetch_offset(2, instr.get_mode(1)),
                self.fetch_offset(3, Mode::VALUE) as P,
            ),
            99 => EXIT,
            _ => panic!("Bad instruction {:}", &instr.code),
        }
    }

    fn next_n(&self, n: usize) -> Memory {
        self.memory[self.eip..self.eip + n].to_vec()
    }

    fn apply(&mut self, opcode: Opcode) {
        let mut last = Step {
            opcode,
            raw: Vec::new(),
        };
        match opcode {
            INIT => unimplemented!(),
            ADD(arg1, arg2, output) => {
                last.raw = self.next_n(4);
                self.memory[output] = arg1 + arg2;
                self.eip += 4;
            }
            MUL(arg1, arg2, output) => {
                last.raw = self.next_n(4);
                self.memory[output] = arg1 * arg2;
                self.eip += 4;
            }
            EXIT => {
                self.state = HALTED;
            }
            INPUT(p) => {
                last.raw = self.next_n(2);
                self.memory[p] = self.inputs.pop().unwrap();
                self.eip += 2;
            }
            JMPT(c, eip) => {
                last.raw = self.next_n(3);
                self.eip = match c {
                    0 => self.eip + 3,
                    _ => eip,
                };
            }
            JMPF(c, eip) => {
                last.raw = self.next_n(3);
                self.eip = match c {
                    0 => eip,
                    _ => self.eip + 3,
                };
            }
            LT(arg1, arg2, output) => {
                last.raw = self.next_n(4);
                if arg1 < arg2 {
                    self.memory[output] = 1
                } else {
                    self.memory[output] = 0;
                }
                self.eip += 4;
            }
            EQ(arg1, arg2, output) => {
                last.raw = self.next_n(4);
                if arg1 == arg2 {
                    self.memory[output] = 1
                } else {
                    self.memory[output] = 0;
                }
                self.eip += 4;
            }
            OUTPUT(c) => {
                last.raw = self.next_n(2);
                self.outputs.push(c);
                self.eip += 2;
            }
        }
        self.last = last;
        self.counter += 1;
    }

    fn step_mut(&mut self) {
        let opcode = self.extract_opcode();
        self.apply(opcode);
    }

    fn run_until(&self, limit: Option<usize>) -> Self {
        let mut result = self.clone();
        let max_counter = match limit {
            Some(l) => l,
            None => usize::max_value(),
        };
        while result.state == RUNNING && result.counter <= max_counter {
            result.step_mut();
        }
        result
    }

    fn run(&self) -> Self {
        self.run_until(None)
    }
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut codes: Memory = Vec::new();
    for line in stdin.lock().lines() {
        let line = String::from(line?);
        let tokens: Vec<&str> = line.split(",").collect();
        for token in tokens.iter() {
            codes.push(token.parse().unwrap());
        }
    }
    let program = Program::new(codes);

    let mut part1 = program.clone();
    part1.inputs.push(1);
    let result = part1.run();
    println!("day 1: {:?}", result);

    let mut part2 = program.clone();
    part2.inputs.push(5);
    let result = part2.run();
    println!("day 2: {:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut program = Program::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![999]);

        let mut program = Program::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![1000]);

        let mut program = Program::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        program.inputs.push(9);
        let result = program.run();
        assert_eq!(result.outputs, vec![1001]);
    }

    #[test]
    fn test_equal_position() {
        let mut program = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_lt_position() {
        let mut program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(9);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_lt_immediate() {
        let mut program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(9);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_equal_immediate() {
        let mut program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_jmp_position() {
        let mut program = Program::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        program.inputs.push(0);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Program::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        program.inputs.push(99);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);
    }

    #[test]
    fn test_jmp_immediate() {
        let mut program = Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        program.inputs.push(0);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        program.inputs.push(99);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);
    }

    #[test]
    fn test_input_output() {
        let mut program = Program::new(vec![3, 0, 4, 0, 99]);
        program.inputs.push(1337);
        assert_eq!(program.inputs, vec![1337]);
        assert_eq!(program.outputs, Vec::new());
        let result = program.run();
        assert_eq!(result.inputs, Vec::new());
        assert_eq!(result.outputs, vec![1337]);

        let mut program = Program::new(vec![3, 2, 0, 0, 99]);
        program.inputs.push(4);
        let result = program.run();
        assert_eq!(result.outputs, vec![3]);
    }
}
