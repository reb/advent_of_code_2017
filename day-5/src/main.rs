use std::fs::File;
use std::io::prelude::*;

fn convert(input: &String) -> Vec<i32> {
    input.lines()
        .map(|item| item.to_string().parse::<i32>()
             .expect("Item could not be parsed to i32"))
        .collect::<Vec<i32>>()
}

fn run_instructions(input: &String, update: &Fn(&mut i32)) -> i32 {
    let mut jump_offsets = convert(input);
    let length = jump_offsets.len();
    let mut steps = 0;
    let mut instruction = 0;
    
    loop {
        if instruction.ge(&length) {
            break;
        }

        let jump = jump_offsets[instruction];
        update(&mut jump_offsets[instruction]);
        instruction = ((instruction as i32) + jump) as usize;
        steps += 1;
    }

    return steps;
}


fn update(instruction: &mut i32) {
    *instruction += 1;
}

fn stranger_update(instruction: &mut i32) {
    if (*instruction).ge(&3) {
        *instruction -= 1;
    } else {
        *instruction += 1;
    }
}

fn main() {
    let filename = "input.txt";

    let mut file = File::open(filename)
        .expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let steps = run_instructions(&contents, &update);
    println!("It took {} steps to escape the maze", steps);

    let stranger_steps = run_instructions(&contents, &stranger_update);
    println!("It took {} steps to escape the maze with stranger jumps", stranger_steps);
}
