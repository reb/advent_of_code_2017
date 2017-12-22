use std::collections::HashMap;
use std::cmp;

fn get_input() -> Vec<Instruction> {
    const INPUT: &str = include_str!("input.txt");
    convert(INPUT)
}

fn convert(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| line.split_whitespace()
             .collect::<Vec<&str>>())
        .map(|instruction| { 
            Instruction { 
                action: Operation {
                    register: instruction.get(0).unwrap().to_string(),
                    operator: instruction.get(1).unwrap().to_string(),
                    variable: instruction.get(2).unwrap().parse::<i32>().unwrap(),
                },
                test: Operation {
                    register: instruction.get(4).unwrap().to_string(),
                    operator: instruction.get(5).unwrap().to_string(),
                    variable: instruction.get(6).unwrap().parse::<i32>().unwrap()
                }
            }
        })
        .collect()
}

pub struct Instruction {
    pub action: Operation,
    pub test: Operation,
}

pub struct Operation {
    pub register: String,
    pub operator: String,
    pub variable: i32
}

fn main() {
    let input = get_input();

    let mut registers = HashMap::new();
    let mut max_during = 0;

    for instruction in input {
        // do the test, skip to the next if it fails
        let current_test_variable = *registers.entry(instruction.test.register).or_insert(0);
        if !match &*instruction.test.operator {
            "=="    => current_test_variable == instruction.test.variable,
            "!="    => current_test_variable != instruction.test.variable,
            ">"     => current_test_variable > instruction.test.variable,
            "<"     => current_test_variable < instruction.test.variable,
            ">="    => current_test_variable >= instruction.test.variable,
            "<="    => current_test_variable <= instruction.test.variable,
            _       => false,
        } { continue; }

        // execute the action
        let current_action_entry = registers.entry(instruction.action.register).or_insert(0);
        match &*instruction.action.operator {
            "inc"   => *current_action_entry += instruction.action.variable,
            "dec"   => *current_action_entry -= instruction.action.variable,
            _       => (),
        }

        // track the biggest value during the process
        max_during = cmp::max(max_during, *current_action_entry);
    }

    let max = registers.values().max().unwrap();
    println!("The biggest value in the registers is {}", max);
    println!("The biggest value in the registers during the process was {}", max_during);
}
