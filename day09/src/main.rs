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
