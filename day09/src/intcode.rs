#[macro_use]
extern crate log;

use std::fmt;
use Opcode::*;
use State::*;
use Value::*;

type C = i64;
pub type Memory = Vec<C>;

const MEMORY_LENGTH: usize = 2048;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Pointer(usize),
    Immediate(C),
    Relative(C),
}

impl Value {
    fn from(number: C, flag: Option<char>) -> Self {
        match flag {
            Some('0') => Pointer(number as usize),
            Some('1') => Immediate(number),
            Some('2') => Relative(number),
            Some(_) => panic!("Unexpected arg mode"),
            None => Pointer(number as usize),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    Init,
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    SetRelativeBase,
    Exit,
}

impl Opcode {
    fn new(code: C) -> Opcode {
        match code {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpTrue,
            6 => JumpFalse,
            7 => LessThan,
            8 => Equals,
            9 => SetRelativeBase,
            99 => Exit,
            _ => panic!("Bad instruction {:}", &code),
        }
    }

    fn len(&self) -> usize {
        match self {
            Init => 1,
            Add => 4,
            Multiply => 4,
            Input => 2,
            Output => 2,
            JumpTrue => 3,
            JumpFalse => 3,
            LessThan => 4,
            Equals => 4,
            SetRelativeBase => 2,
            Exit => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    args: Vec<Value>,
    raw: Memory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Running,
    AwaitingInput,
    Halted,
}

#[derive(Clone)]
pub struct Computer {
    pub memory: Memory,
    pub eip: usize,
    pub counter: usize,
    pub relative_base: C,
    pub state: State,
    pub last: Instruction,
    pub inputs: Vec<C>,
    pub outputs: Vec<C>,
}

impl fmt::Debug for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ {:04} ] {:?}::{{ eip={} rel={} in={:?} out={:?} }}",
            self.counter, self.state, self.eip, self.relative_base, self.inputs, self.outputs,
        )
    }
}

impl Computer {
    pub fn new(memory: Memory) -> Self {
        let mut extended_memory = memory.clone();
        extended_memory.resize(MEMORY_LENGTH, 0);
        Computer {
            memory: extended_memory,
            eip: 0,
            counter: 0,
            relative_base: 0,
            state: Running,
            last: Instruction {
                opcode: Init,
                raw: Vec::new(),
                args: Vec::new(),
            },
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn next_n(&self, n: usize) -> Memory {
        self.memory[self.eip..self.eip + n].to_vec()
    }

    fn read(&self, location: Value) -> C {
        match location {
            Immediate(value) => {
                debug!("-- R: {} from {:?}", value, location);
                value
            }
            Pointer(addr) => {
                debug!("-- R: {} from {:?}", self.memory[addr], location);
                self.memory[addr]
            }
            Relative(offset) => {
                let addr = (self.relative_base + offset) as usize;
                debug!(
                    "-- R: {} from {:?} (addr={})",
                    self.memory[addr], location, addr
                );
                self.memory[addr]
            }
        }
    }

    fn write(&mut self, location: Value, value: C) {
        match location {
            Immediate(_) => panic!("Can't write a value in immediate mode"),
            Pointer(addr) => {
                debug!("-- W: {} at {:?}", value, location);
                self.memory[addr] = value
            }
            Relative(offset) => {
                let addr = (self.relative_base + offset) as usize;
                debug!("-- W: {} at {:?} (addr={})", value, location, addr);
                self.memory[addr] = value
            }
        }
    }

    fn extract_instruction(&self) -> Instruction {
        let s = format!("{:}", &self.memory[self.eip]);
        let (args_part, code_part) = match s.len() {
            1 => ("", &s[..]),
            _ => s.split_at(s.len() - 2),
        };
        let code_raw = code_part.clone().parse().unwrap();
        let opcode: Opcode = Opcode::new(code_raw);
        let raw = self.next_n(opcode.len());
        let args: Vec<Value> = raw
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, &a)| Value::from(a, args_part.chars().rev().nth(i)))
            .collect();
        Instruction { raw, opcode, args }
    }

    fn apply(&mut self, instr: Instruction) {
        let mut next_eip = self.eip + instr.opcode.len();
        match instr.opcode {
            Init => unimplemented!(),
            Add => {
                let result = self.read(instr.args[0]) + self.read(instr.args[1]);
                self.write(instr.args[2], result);
            }
            Multiply => {
                let result = self.read(instr.args[0]) * self.read(instr.args[1]);
                self.write(instr.args[2], result);
            }
            Exit => {
                self.state = Halted;
            }
            Input => {
                if let Some(input) = self.inputs.pop() {
                    self.write(instr.args[0], input);
                    self.state = State::Running;
                } else {
                    // do nothing and wait
                    next_eip = self.eip;
                    self.state = State::AwaitingInput;
                }
            }
            JumpTrue => {
                let value = self.read(instr.args[0]);
                if value != 0 {
                    let jump_to = self.read(instr.args[1]);
                    next_eip = jump_to as usize;
                }
            }
            JumpFalse => {
                let value = self.read(instr.args[0]);
                if value == 0 {
                    let jump_to = self.read(instr.args[1]);
                    next_eip = jump_to as usize;
                }
            }
            LessThan => {
                let less_than = self.read(instr.args[0]) < self.read(instr.args[1]);
                self.write(
                    instr.args[2],
                    match less_than {
                        true => 1,
                        false => 0,
                    },
                );
            }
            Equals => {
                let equal = self.read(instr.args[0]) == self.read(instr.args[1]);
                self.write(
                    instr.args[2],
                    match equal {
                        true => 1,
                        false => 0,
                    },
                );
            }
            Output => {
                let value = self.read(instr.args[0]);
                self.outputs.push(value);
            }
            SetRelativeBase => {
                let offset = self.read(instr.args[0]);
                debug!(
                    "-- B: {} ({} + {})",
                    self.relative_base + offset,
                    self.relative_base,
                    offset
                );
                self.relative_base += offset;
            }
        }
        self.eip = next_eip;
        self.last = instr;
        self.counter += 1;
    }

    pub fn step_mut(&mut self) {
        let instr = self.extract_instruction();
        debug!("-- X: {:?}", &instr);
        self.apply(instr);
    }

    pub fn run_until(&self, limit: Option<usize>) -> Self {
        let mut result = self.clone();
        let max_counter = match limit {
            Some(l) => l,
            None => usize::max_value(),
        };
        while result.state == Running && result.counter <= max_counter {
            result.step_mut();
        }
        result
    }

    pub fn run(&self) -> Self {
        self.run_until(None)
    }
}

#[cfg(test)]
mod tests;
