use std::io;
use std::io::{BufRead, Error};
use Opcode::*;
use State::*;

type C = i32; // contents
type P = usize; // pointers
type Memory = Vec<C>;

#[derive(Debug, Clone)]
enum Opcode {
    ADD(C, C, P),
    MUL(C, C, P),
    SAVE(C, P),
    OUTPUT(P),
    EXIT,
}

enum Mode {
    PARAMETER,
    IMMEDIATE,
}

#[derive(Debug, Clone)]
enum State {
    RUNNING,
    HALTED,
}

#[derive(Debug, Clone)]
struct Program {
    memory: Memory,
    eip: P,
    counter: C,
    state: State,
}

impl Program {
    fn new(memory: Memory) -> Self {
        Program {
            memory,
            eip: 0,
            counter: 0,
            state: RUNNING,
        }
    }

    fn apply(program: &mut Self, opcode: Opcode) {
        program.counter += 1;
        match opcode {
            ADD(arg1, arg2, output) => {
                program.memory[output] = arg1 + arg2;
                program.eip += 4;
            }
            MUL(arg1, arg2, output) => {
                program.memory[output] = arg1 * arg2;
                program.eip += 4;
            }
            EXIT => {
                program.state = HALTED;
            }
            _ => panic!(),
        }
    }

    fn extract_opcode(program: &Program) -> Opcode {
        let s = String::from(program.memory[program.eip]);
        let instr: i32 = (&s[s.len() - 2..]).parse::<i32>().unwrap();
        let mut modes: Vec<Mode> = s
            .chars()
            .rev()
            .skip(2)
            .map(|c| match c {
                '0' => Mode::PARAMETER,
                '1' => Mode::IMMEDIATE,
                _ => panic!("Invalid mode"),
            })
            .collect();
        match instr {
            1 => ADD(
                program.memory[program.eip + 1] as C,
                program.memory[program.eip + 2] as C,
                program.memory[program.eip + 3] as P,
            ),
            2 => MUL(
                program.memory[program.eip + 1] as C,
                program.memory[program.eip + 2] as C,
                program.memory[program.eip + 3] as P,
            ),
            99 => EXIT,
            _ => panic!(
                "Unexpected instruction '{:}' at eip={:}",
                instr, program.eip
            ),
        }
    }

    fn step(program: Self) -> Self {
        let mut r = program.clone();
        let opcode = Program::extract_opcode(&r);
        Program::apply(&mut r, opcode);
        r
    }

    fn run(program: Self) -> Self {
        let mut result = program.clone();
        loop {
            match result.state {
                HALTED => break,
                RUNNING => {
                    result = Program::step(result);
                }
            }
        }
        result
    }
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut codes: Memory = Vec::new();
    for program in stdin.lock().lines() {
        let program = String::from(program?);
        let tokens: Vec<&str> = program.split(",").collect();
        for token in tokens.iter() {
            codes.push(token.parse().unwrap());
        }
    }
    codes[1] = 12;
    codes[2] = 2;
    let program = Program::new(codes);

    // day 1
    let result = Program::run(program.clone());
    println!("day 1: {:?}", result.memory[0]);

    // day 2
    for noun in 0..99 {
        for verb in 0..99 {
            let mut candidate = program.clone();
            candidate.memory[1] = noun;
            candidate.memory[2] = verb;
            let result = Program::run(candidate);
            if result.memory[0] == 19690720 {
                let answer = 100 * noun + verb;
                println!("day 2: answer={:} [noun={:} verb={:}]", answer, noun, verb);
                return Ok(());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let program = Program::new(vec![1, 0, 0, 0, 99]);
        let result = Program::run(program);
        assert_eq!(result.memory, vec![2, 0, 0, 0, 99]);

        let program = Program::new(vec![2, 3, 0, 3, 99]);
        let result = Program::run(program);
        assert_eq!(result.memory, vec![2, 3, 0, 6, 99]);

        let program = Program::new(vec![2, 4, 4, 5, 99, 0]);
        let result = Program::run(program);
        assert_eq!(result.memory, vec![2, 4, 4, 5, 99, 9801]);

        let program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        let result = Program::run(program);
        assert_eq!(result.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
