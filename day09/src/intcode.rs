use std::collections::HashMap;
use Opcode::*;
use State::*;

type C = i32;
pub type Memory = Vec<C>;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    POSITION,
    IMMEDIATE,
    RELATIVE,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    raw: Memory,
    opcode: Opcode,
    modes: HashMap<usize, Mode>,
    args: Memory,
    values: Memory,
}

impl Instruction {
    fn opcode(code: C, v: &Memory) -> Opcode {
        match code {
            1 => ADD(v[0], v[1], v[2]),
            2 => MUL(v[0], v[1], v[2]),
            3 => INPUT(v[0]),
            4 => OUTPUT(v[0]),
            5 => JMPT(v[0], v[1]),
            6 => JMPF(v[0], v[1]),
            7 => LT(v[0], v[1], v[2]),
            8 => EQ(v[0], v[1], v[2]),
            99 => EXIT,
            _ => panic!("Bad instruction {:}", &code),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    INIT,
    ADD(C, C, C),
    MUL(C, C, C),
    INPUT(C),
    OUTPUT(C),
    JMPT(C, C),
    JMPF(C, C),
    LT(C, C, C),
    EQ(C, C, C),
    EXIT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    RUNNING,
    HALTED,
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub memory: Memory,
    pub eip: usize,
    pub counter: usize,
    pub state: State,
    pub last: Instruction,
    pub inputs: Vec<C>,
    pub outputs: Vec<C>,
}

impl Computer {
    pub fn new(memory: Memory) -> Self {
        Computer {
            memory,
            eip: 0,
            counter: 0,
            state: RUNNING,
            last: Instruction {
                opcode: INIT,
                raw: Vec::new(),
                modes: HashMap::new(),
                args: Vec::new(),
                values: Vec::new(),
            },
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn next_n(&self, n: usize) -> Memory {
        self.memory[self.eip..self.eip + n].to_vec()
    }

    fn extract_instruction(&self) -> Instruction {
        let s = format!("{:}", self.memory[self.eip]);

        // grab the opcode digits
        let n = s.len();
        let code: i32 = match s.len() {
            1 => &s[..],
            2 => &s[..],
            _ => &s[n - 2..],
        }
        .parse()
        .unwrap();

        // take the right number of raw codes depending on what we extracted
        let raw = self.next_n(match code {
            1 => 4,
            2 => 4,
            3 => 2,
            4 => 2,
            5 => 3,
            6 => 3,
            7 => 4,
            8 => 4,
            99 => 1,
            _ => panic!("Bad opcode: {}", code),
        });

        // args are the raw instruction codes after the first
        let args: Memory = raw.clone().into_iter().skip(1).collect();

        // parse which parameter modes are in use
        let mode_flags: Vec<char> = s.chars().rev().skip(2).collect();
        let mut modes: HashMap<usize, Mode> = (0..args.len())
            .map(|i| match mode_flags.get(i) {
                Some('0') => (i, Mode::POSITION),
                Some('1') => (i, Mode::IMMEDIATE),
                Some(a) => panic!("Invalid mode: {:}", a),
                None => (i, Mode::POSITION),
            })
            .collect();

        let _replaced_output_mode: Option<Mode> = match code {
            1 => modes.insert(args.len() - 1, Mode::IMMEDIATE),
            2 => modes.insert(args.len() - 1, Mode::IMMEDIATE),
            3 => modes.insert(args.len() - 1, Mode::IMMEDIATE),
            7 => modes.insert(args.len() - 1, Mode::IMMEDIATE),
            8 => modes.insert(args.len() - 1, Mode::IMMEDIATE),
            _ => None,
        };

        // read the actual values
        let values: Memory = args
            .iter()
            .enumerate()
            .map(
                |(i, &a)| match modes.get(&i).expect("Did not find a mode") {
                    Mode::IMMEDIATE => a,
                    Mode::POSITION => self.memory[a as usize],
                },
            )
            .collect();

        let opcode = Instruction::opcode(code, &values);
        Instruction {
            raw,
            opcode,
            args,
            values,
            modes,
        }
    }

    fn apply(&mut self, instr: Instruction) {
        match instr.opcode {
            INIT => unimplemented!(),
            ADD(arg1, arg2, output) => {
                self.memory[output as usize] = arg1 + arg2;
                self.eip += 4;
            }
            MUL(arg1, arg2, output) => {
                self.memory[output as usize] = arg1 * arg2;
                self.eip += 4;
            }
            EXIT => {
                self.state = HALTED;
            }
            INPUT(p) => {
                if let Some(input) = self.inputs.pop() {
                    self.memory[p as usize] = input;
                    self.eip += 2;
                }
            }
            JMPT(c, eip) => {
                self.eip = match c {
                    0 => self.eip + 3,
                    _ => eip as usize,
                };
            }
            JMPF(c, eip) => {
                self.eip = match c {
                    0 => eip as usize,
                    _ => self.eip + 3,
                };
            }
            LT(arg1, arg2, output) => {
                if arg1 < arg2 {
                    self.memory[output as usize] = 1
                } else {
                    self.memory[output as usize] = 0;
                }
                self.eip += 4;
            }
            EQ(arg1, arg2, output) => {
                if arg1 == arg2 {
                    self.memory[output as usize] = 1
                } else {
                    self.memory[output as usize] = 0;
                }
                self.eip += 4;
            }
            OUTPUT(c) => {
                self.outputs.push(c);
                self.eip += 2;
            }
        }
        self.last = instr;
        self.counter += 1;
    }

    pub fn step_mut(&mut self) {
        let instr = self.extract_instruction();
        self.apply(instr);
    }

    pub fn run_until(&self, limit: Option<usize>) -> Self {
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

    pub fn run(&self) -> Self {
        self.run_until(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut program = Computer::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![999]);

        let mut program = Computer::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![1000]);

        let mut program = Computer::new(vec![
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
        let mut program = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_lt_position() {
        let mut program = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.inputs.push(9);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_lt_immediate() {
        let mut program = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(9);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_equal_immediate() {
        let mut program = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(8);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);

        let mut program = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        program.inputs.push(7);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);
    }

    #[test]
    fn test_jmp_position() {
        let mut program = Computer::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        program.inputs.push(0);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Computer::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        program.inputs.push(99);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);
    }

    #[test]
    fn test_jmp_immediate() {
        let mut program = Computer::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        program.inputs.push(0);
        let result = program.run();
        assert_eq!(result.outputs, vec![0]);

        let mut program = Computer::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        program.inputs.push(99);
        let result = program.run();
        assert_eq!(result.outputs, vec![1]);
    }

    #[test]
    fn test_input_output() {
        let mut program = Computer::new(vec![3, 0, 4, 0, 99]);
        program.inputs.push(1337);
        assert_eq!(program.inputs, vec![1337]);
        assert_eq!(program.outputs, Vec::new());
        let result = program.run();
        assert_eq!(result.inputs, Vec::new());
        assert_eq!(result.outputs, vec![1337]);

        let mut program = Computer::new(vec![3, 2, 0, 0, 99]);
        program.inputs.push(4);
        let result = program.run();
        assert_eq!(result.outputs, vec![3]);
    }
}
