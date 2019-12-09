use std::io;
use std::io::{BufRead, Error};
use Opcode::*;
use State::*;

const DEBUG: bool = true;

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


#[derive(Debug, Clone, PartialEq, Eq)]
enum Opcode {
    INIT,
    ADD(C, C, P),
    MUL(C, C, P),
    INPUT(P),
    OUTPUT(P),
    EXIT,
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
    last: Opcode,
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
            last: INIT,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn fetch(&self, value: C, mode: Mode) -> C {
        match mode {
            Mode::VALUE => value,
            Mode::REFERENCE => self.memory[value as P]
        }
    }

    fn fetch_offset(&self, offset: usize, mode: Mode) -> C {
        let value: C = self.memory[self.eip + offset];
        self.fetch(value, mode)
    }

    fn extract_opcode(&self) -> Opcode {
        let instr = Instruction::from(self.memory[self.eip]);
        if DEBUG {
            println!("Extracted instruction: {:?}", instr);
        }
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
            3 => INPUT(
                self.fetch_offset(1, Mode::VALUE) as P,
            ),
            4 => OUTPUT(
                self.fetch_offset(1, Mode::VALUE) as P,
            ),
            99 => EXIT,
            _ => panic!("Bad instruction {:}", &instr.code)
        }
    }

    fn apply(&mut self, opcode: Opcode) {
        if DEBUG {
            println!("Applying: {:?}", opcode);
        }
        match opcode {
            INIT => unimplemented!(),
            ADD(arg1, arg2, output) => {
                self.memory[output] = arg1 + arg2;
                self.eip += 4;
            }
            MUL(arg1, arg2, output) => {
                self.memory[output] = arg1 * arg2;
                self.eip += 4;
            }
            EXIT => {
                self.state = HALTED;
            }
            INPUT(p) => {
                self.memory[p] = self.inputs.pop().unwrap();
                self.eip += 2;
            }
            OUTPUT(p) => {
                self.outputs.push(self.memory[p]);
                self.eip += 2;
            }
        }
        self.last = opcode;
        self.counter += 1;
    }

    fn step_mut(&mut self) {
        let opcode = self.extract_opcode();
        self.apply(opcode);
    }

    fn run_until(&self, limit: Option<usize>) -> Self {
        let mut result = self.clone();
        if DEBUG {
            println!("{:?}", result);
        }
        let max_counter = match limit {
            Some(l) => l,
            None => usize::max_value()
        };
        while result.state == RUNNING && result.counter <= max_counter {
            result.step_mut();
            if DEBUG {
                println!("{:?}", result);
            }
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

    // day 1
    let mut result = program.clone();
    result.inputs.push(1);
    while result.state != HALTED {
        result.step_mut();
        if let Some(&latest) = result.outputs.last() {
            if latest != 0 {
                break
            }
        }
    }
    println!("day 1: {:?}", result.memory[0]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = Program::new(vec![1, 0, 0, 0, 99]).run();
        assert_eq!(result.memory, vec![1, 2, 0, 0, 99]);

        let result = Program::new(vec![2, 3, 0, 3, 99]).run();
        assert_eq!(result.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_extract_opcode() {
        let program = Program::new(vec![1002, 4, 3, 4, 33]);
        let opcode = program.extract_opcode();
        assert_eq!(opcode, MUL(33, 3, 33));

        let program = Program::new(vec![1102, 4, 3, 4, 33]);
        let opcode = program.extract_opcode();
        assert_eq!(opcode, MUL(4, 3, 33));
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
