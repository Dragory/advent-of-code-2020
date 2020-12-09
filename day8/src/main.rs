// https://adventofcode.com/2020/day/8

use std::fs;
use std::collections::HashSet;

enum Operation {
    Nop,
    Acc,
    Jmp
}

struct Instruction {
    operation: Operation,
    value: i32
}

struct ExecutionResult {
    accumulator: i32,
    success: bool
}

fn parse_instruction(instruction: &str) -> Instruction {
    let parts = instruction.split(" ").collect::<Vec<_>>();
    let opname = parts[0];
    let raw_value = parts[1];
    let value = raw_value.parse::<i32>().unwrap();

    let operation = match opname {
        "nop" => Operation::Nop,
        "acc" => Operation::Acc,
        "jmp" => Operation::Jmp,
        _ => panic!("Invalid opname {}", opname)
    };

    return Instruction {
        operation,
        value: value
    };
}

fn execute_instructions(instructions: &Vec<&Instruction>) -> ExecutionResult {
    let mut accumulator: i32 = 0;
    let mut instruction_index: i32 = 0;
    let mut executed_lines: HashSet<usize> = HashSet::new();
    let mut success = false;

    loop {
        executed_lines.insert(instruction_index as usize);

        let instruction = &instructions[instruction_index as usize];
        match instruction.operation {
            Operation::Acc => {
                accumulator += instruction.value;
                instruction_index += 1;
            },
            Operation::Jmp => {
                instruction_index += instruction.value;
            },
            Operation::Nop => {
                instruction_index += 1;
            }
        }

        if executed_lines.contains(&(instruction_index as usize)) {
            break;
        }

        if instruction_index as usize >= instructions.len() {
            success = true;
            break;
        }
    }

    return ExecutionResult {
        accumulator,
        success
    };
}

// "Immediately before any instruction is executed a second time, what value is in the accumulator?"
fn part1(instructions: &Vec<Instruction>) -> i32 {
    let result = execute_instructions(&instructions.iter().collect::<Vec<_>>());
    return result.accumulator;
}

// "Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?"
fn part2(instructions: &Vec<Instruction>) -> i32 {
    for i in 0..instructions.len() {
        match instructions[i].operation {
            Operation::Acc => continue,
            _ => {
                let mut copy = instructions.iter().take(i).collect::<Vec<_>>();
                let tail = instructions.iter().skip(i + 1);

                let replacement_instruction = Instruction {
                    operation: match instructions[i].operation {
                        Operation::Jmp => Operation::Nop,
                        Operation::Nop => Operation::Jmp,
                        _ => panic!("???")
                    },
                    value: instructions[i].value
                };

                copy.push(&replacement_instruction);
                copy.extend(tail);

                let result = execute_instructions(&copy);
                if result.success {
                    return result.accumulator;
                }
            }
        }
    }

    panic!("No solution for part 2");
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    let lines = input.lines();
    let instructions = lines.map(|line| parse_instruction(line)).collect::<Vec<_>>();

    println!("Part 1:");
    let part1_result = part1(&instructions);
    println!("Result: {}", part1_result);

    println!("Part 2:");
    let part2_result = part2(&instructions);
    println!("Result: {}", part2_result);
}
