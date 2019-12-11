use super::*;

#[test]
fn test_run() {
    let codes = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    let mut program = Computer::new(codes.clone());
    program.inputs.push(7);
    let result = program.run();
    assert_eq!(result.outputs, vec![999]);

    let mut program = Computer::new(codes.clone());
    program.inputs.push(8);
    let result = program.run();
    assert_eq!(result.outputs, vec![1000]);

    let mut program = Computer::new(codes.clone());
    program.inputs.push(9);
    let result = program.run();
    assert_eq!(result.outputs, vec![1001]);
}

#[test]
fn test_add_and_mul() {
    let program = vec![1, 0, 0, 0, 99];
    let result = Computer::new(program.clone()).run();
    assert_eq!(&result.memory[..program.len()], &vec![2, 0, 0, 0, 99][..]);

    let program = vec![2, 3, 0, 3, 99];
    let result = Computer::new(program.clone()).run();
    assert_eq!(&result.memory[..program.len()], &vec![2, 3, 0, 6, 99][..]);

    let program = vec![2, 4, 4, 5, 99, 0];
    let result = Computer::new(program.clone()).run();
    assert_eq!(
        &result.memory[..program.len()],
        &vec![2, 4, 4, 5, 99, 9801][..]
    );

    let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let result = Computer::new(program.clone()).run();
    assert_eq!(
        &result.memory[..program.len()],
        &vec![30, 1, 1, 4, 2, 5, 6, 0, 99][..]
    );
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

#[test]
fn test_day5_modes_and_jumps() {
    // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    let mut program = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    program.inputs.push(8);
    let result = program.run();
    assert_eq!(result.outputs, vec![1]);

    let mut program = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    program.inputs.push(1337);
    let result = program.run();
    assert_eq!(result.outputs, vec![0]);

    // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    let mut program = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    program.inputs.push(7);
    let result = program.run();
    assert_eq!(result.outputs, vec![1]);

    let mut program = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    program.inputs.push(8);
    let result = program.run();
    assert_eq!(result.outputs, vec![0]);

    // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    let mut program = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    program.inputs.push(8);
    let result = program.run();
    assert_eq!(result.outputs, vec![1]);

    let mut program = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    program.inputs.push(1337);
    let result = program.run();
    assert_eq!(result.outputs, vec![0]);

    // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    let mut program = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    program.inputs.push(7);
    let result = program.run();
    assert_eq!(result.outputs, vec![1]);

    let mut program = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    program.inputs.push(8);
    let result = program.run();
    assert_eq!(result.outputs, vec![0]);

    //Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
    let mut program = Computer::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);
    program.inputs.push(0);
    let result = program.run();
    assert_eq!(result.outputs, vec![0]);

    let mut program = Computer::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);
    program.inputs.push(1337);
    let result = program.run();
    assert_eq!(result.outputs, vec![1]);

    let mut program = Computer::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
    program.inputs.push(0);
    let result = program.run();
    assert_eq!(result.outputs, vec![0]);

    let mut program = Computer::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
    program.inputs.push(1337);
    let result = program.run();
    assert_eq!(result.outputs, vec![1]);
}

#[test]
fn test_day9_part1() {
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
