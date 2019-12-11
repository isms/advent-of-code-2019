extern crate env_logger;

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
    env_logger::init();
    let program = read_program()?;

    let mut computer = program.clone();
    computer.inputs.push(1);
    let result = computer.run();
    println!("part 1: {:?}", result.outputs.get(0));

    let mut computer = program.clone();
    computer.inputs.push(2);
    let result = computer.run();
    println!("part 2: {:?}", result.outputs.get(0));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let computer = Computer::new(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        let result = computer.run();
        assert_eq!(
            result.outputs,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        let computer = Computer::new(vec![104, 1125899906842624, 99]);
        let result = computer.run();
        assert_eq!(result.outputs, vec![1125899906842624]);

        let computer = Computer::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        let result = computer.run();
        assert_eq!(format!("{}", result.outputs[0]).len(), 16);
    }
}
