use std::io::{self, BufRead, Error};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    ADD = 1,
    MUL = 2,
    EXIT = 99,
}

impl From<isize> for Instruction {
    fn from(number: isize) -> Self {
        match number {
            1 => Instruction::ADD,
            2 => Instruction::MUL,
            99 => Instruction::EXIT,
            _ => panic!(format!("Bad instruction: {}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Opcode {
    instr: Instruction,
    input1: usize,
    input2: usize,
    output_address: usize,
}

impl Opcode {
    fn new(instr: isize, input1: isize, input2: isize, output_address: isize) -> Self {
        Opcode {
            instr: Instruction::from(instr),
            input1: input1 as usize,
            input2: input2 as usize,
            output_address: output_address as usize,
        }
    }
}

fn run(program: Vec<isize>) -> Vec<isize> {
    let mut result = program.clone();
    let mut eip = 0;
    loop {
        if Instruction::from(result[eip]) == Instruction::EXIT {
            break result;
        }
        let opcode = Opcode::new(
            result[eip],
            result[eip + 1],
            result[eip + 2],
            result[eip + 3],
        );
        match opcode.instr {
            Instruction::ADD => {
                result[opcode.output_address] = result[opcode.input1] + result[opcode.input2]
            }
            Instruction::MUL => {
                result[opcode.output_address] = result[opcode.input1] * result[opcode.input2]
            }
            Instruction::EXIT => break result,
        }
        eip += 4;
    }
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    // initialize with invalid input
    let mut codes: Vec<isize> = vec![-1, -1, -1, -1];
    for program in stdin.lock().lines() {
        let program = String::from(program?);
        let tokens: Vec<&str> = program.split(",").collect();
        codes = tokens.iter().map(|x| x.parse().unwrap()).collect();
    }
    codes[1] = 12;
    codes[2] = 2;

    // day 1
    let result = run(codes.clone());
    println!("day 1: {:?}", result);

    // day 2
    for noun in 0..99 {
        for verb in 0..99 {
            let mut candidate = codes.clone();
            candidate[1] = noun;
            candidate[2] = verb;
            let result = run(candidate);
            if result[0] == 19690720 {
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
    fn test_instruction() {
        assert_eq!(Instruction::from(1), Instruction::ADD);
        assert_eq!(Instruction::from(2), Instruction::MUL);
        assert_eq!(Instruction::from(99), Instruction::EXIT);
    }

    #[test]
    fn test_run() {
        let program = vec![1, 0, 0, 0, 99];
        let result = run(program);
        assert_eq!(result, vec![2, 0, 0, 0, 99]);

        let program = vec![2, 3, 0, 3, 99];
        let result = run(program);
        assert_eq!(result, vec![2, 3, 0, 6, 99]);

        let program = vec![2, 4, 4, 5, 99, 0];
        let result = run(program);
        assert_eq!(result, vec![2, 4, 4, 5, 99, 9801]);

        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = run(program);
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
