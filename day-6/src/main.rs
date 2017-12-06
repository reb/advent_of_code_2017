use std::collections::HashSet;


fn get_input() -> Vec<u32> {
    const INPUT: &str = include_str!("input.txt");
    convert(INPUT)
}

fn convert(input: &str) -> Vec<u32> {
    input.lines().next().unwrap()
        .split_whitespace()
        .map(|item| item.parse()
             .expect("Item could not be parsed"))
        .collect()
}

fn redistribute(memory: &Vec<u32>) -> Vec<u32> {
    // get the fullest memory bank location
    let (max_location, max_blocks) = memory.iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, blocks)| blocks)
        .expect("Could not find max");

    // create a clone
    let mut new_memory = memory.clone();
    let mut blocks_left = *max_blocks;
    let mut location = max_location;

    // empty the biggest block
    new_memory[location] = 0;

    // redistribute blocks, starting with the next
    loop {
        // stop if all blocks are redistributed
        if blocks_left.le(&0) {
            break;
        }

        location += 1;

        // wrap around if at the end
        if location.ge(&memory.len()) {
            location = 0;
        }

        new_memory[location] += 1;
        blocks_left -= 1;
    }

    // verify result
    assert_eq!(memory.iter().sum::<u32>(), new_memory.iter().sum());

    new_memory
}

fn main() {
    let mut memory = get_input();
    let mut previous_states = HashSet::new();
    let mut loop_states = HashSet::new();

    loop {
        let new_memory = redistribute(&memory);

        // check if the memory state was reached before
        if previous_states.contains(&new_memory) {
            // if it was check if it was already reach before in the loop
            if loop_states.contains(&new_memory) {
                break;
            }
            // otherwise add it to the loop states
            loop_states.insert(memory);
        } else {
            // otherwise add it to the previous states
            previous_states.insert(memory);
        }

        memory = new_memory;
    }

    println!("It took {} cycles to reach a loop", previous_states.len() + 1);
    println!("And the loop is {} cycles long", loop_states.len() + 1);
}
