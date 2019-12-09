extern crate itertools;

use intcode::State::RUNNING;
use intcode::{Computer, Memory};
use itertools::Itertools;
use std::io;
use std::io::{BufRead, Error};

fn read_program() -> Result<Computer, Error> {
    let stdin = io::stdin();
    let mut codes: Memory = Vec::new();
    for line in stdin.lock().lines() {
        let line = String::from(line?);
        let tokens: Vec<&str> = line.split(",").collect();
        for token in tokens.iter() {
            codes.push(token.parse().unwrap());
        }
    }
    Ok(Computer::new(codes))
}

fn create_chain(computer: &Computer, phases: Vec<i32>) -> Vec<Computer> {
    let mut chain = vec![
        computer.clone(),
        computer.clone(),
        computer.clone(),
        computer.clone(),
        computer.clone(),
    ];
    chain.iter_mut().enumerate().for_each(|(i, computer)| {
        computer.inputs.push(phases[i]);
    });
    chain
}

fn try_phases(computer: &Computer, phases: &Vec<i32>) -> Option<i32> {
    let mut chain = create_chain(&computer, phases.clone());
    chain[0].inputs.insert(0, 0);
    let n = chain.len();
    let mut step: usize = 0;
    let mut i: usize;
    let mut i_prev: usize;
    let mut c: Computer;
    while chain[n - 1].state == RUNNING {
        i = step % n;
        i_prev = match i {
            0 => n - 1,
            _ => i - 1,
        };
        if chain[i].state == RUNNING {
            if let Some(input) = chain[i_prev].outputs.pop() {
                chain[i].inputs.insert(0, input);
            }
            chain[i].step_mut();
        }
        step += 1;
    }
    return chain[n - 1].outputs.pop();
}

fn main() -> Result<(), Error> {
    let computer = read_program()?;

    // part 1
    let mut max = 0;
    for (i, p) in (0..=4).permutations(5).enumerate() {
        if let Some(value) = try_phases(&computer, &p) {
            if value > max {
                println!(
                    "part1: found higher value={} at iter={} ({:?})",
                    value, i, p
                );
                max = value;
            }
        }
    }

    // part 2
    let mut max = 0;
    for (i, p) in (5..=9).permutations(5).enumerate() {
        if let Some(value) = try_phases(&computer, &p) {
            if value > max {
                println!(
                    "part 2: found higher value={} at iter={} ({:?})",
                    value, i, p
                );
                max = value;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let computer = Computer::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        let result = try_phases(&computer, &vec![4, 3, 2, 1, 0]);
        assert_eq!(result, Some(43210));

        let computer = Computer::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        let result = try_phases(&computer, &vec![0, 1, 2, 3, 4]);
        assert_eq!(result, Some(54321));

        let computer = Computer::new(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        let result = try_phases(&computer, &vec![1, 0, 4, 3, 2]);
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part2() {
        let computer = Computer::new(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        let result = try_phases(&computer, &vec![9, 8, 7, 6, 5]);
        assert_eq!(result, Some(139629729));

        let computer = Computer::new(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        let result = try_phases(&computer, &vec![9, 7, 8, 5, 6]);
        assert_eq!(result, Some(18216));
    }
}
