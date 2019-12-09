use intcode::{Computer, Memory};
use std::io;
use std::io::{BufRead, Error};

fn read_program() -> Result<Computer, Error> {
    let stdin = io::stdin();
    let mut codes: Memory = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split(',').collect();
        for token in tokens.into_iter() {
            codes.push(token.parse().unwrap());
        }
    }
    Ok(Computer::new(codes))
}

fn main() -> Result<(), Error> {
    let mut computer = read_program()?;
    computer.inputs.push(1);
    println!("{:?}", computer.run());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let computer = Computer::new(vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
        let result = computer.run();
        assert_eq!(result.outputs, vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
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
}
