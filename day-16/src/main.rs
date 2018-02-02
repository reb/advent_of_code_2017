#[macro_use]
extern crate nom;

use std::time::Instant;
use nom::{alpha, digit};

const PROGRAMS_SIZE: u8 = 16;
const BILLION: u32 = 1000000000;

named!(int<usize>,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8),
        std::str::FromStr::from_str
    )
);

named!(character<char>,
    map_res!(
        map_res!(
            alpha,
            std::str::from_utf8),
        std::str::FromStr::from_str
    )
);

named!(parse_spin<usize>,
    do_parse!(
        tag!("s") >>
        spin: int >>
        (spin)
    )
);

named!(parse_exchange<(usize, usize)>,
    do_parse!(
        tag!("x") >>
        a: int >>
        tag!("/") >>
        b: int >>
        (a, b)
    )
);

named!(parse_partner<(char, char)>,
    do_parse!(
        tag!("p") >>
        a: character >>
        tag!("/") >>
        b: character >>
        (a, b)
    )
);

struct Instruction(Box<Fn(&mut Vec<char>) -> ()>);

fn generate_spin_closure(offset: usize) -> Box<Fn(&mut Vec<char>) -> ()> {
    let index_offset = (PROGRAMS_SIZE as usize) - offset;
    Box::new(move |programs| {
        let end_part: Vec<char> = programs.drain(..index_offset).collect();
        programs.extend(end_part);;
    })
}

fn generate_exchange_closure(a: usize, b: usize) -> Box<Fn(&mut Vec<char>) -> ()> {
    Box::new(move |programs| {
        programs.swap(a, b);
    })
}

fn generate_partner_closure(a: char, b: char) -> Box<Fn(&mut Vec<char>) -> ()> {
    Box::new(move |programs| {
        let (mut index_a, mut index_b) = (0, 0);
        for (index, &program) in programs.iter().enumerate() {
            if program == a {
                index_a = index;
            }
            if program == b {
                index_b = index;
            }
        }
        programs.swap(index_a, index_b);
    })
}

fn get_input() -> Vec<Instruction> {
    const INPUT: &str = include_str!("input.txt");

    INPUT.lines().next().unwrap().split(',')
        .map(|instruction| {
            if let Some(offset) = parse_spin(&instruction.as_bytes()).to_result().ok() {
                return Instruction(generate_spin_closure(offset));
            }
            if let Some((a, b)) = parse_exchange(&instruction.as_bytes()).to_result().ok() {
                return Instruction(generate_exchange_closure(a, b));
            }
            if let Some((a, b)) = parse_partner(&instruction.as_bytes()).to_result().ok() {
                return Instruction(generate_partner_closure(a, b));
            }
            return Instruction(Box::new(move |_| ()));
        })
        .collect()
}

fn dance(instructions: &Vec<Instruction>, iterations: u32) -> String {
    let mut programs: Vec<char> = (0..PROGRAMS_SIZE).map(|i| (('a' as u8) + i) as char).collect();

    let start = programs.clone();
    let mut i = 0;

    while i < iterations {
        for instruction in instructions {
            instruction.0(&mut programs);
        }

        i += 1;
        if programs == start {
            println!("Found a cycle after {} iterations", i);
            i = BILLION - BILLION % i;
            println!("Jumped to iteration {} to skip all complete cycles", i);
        }
    }

    programs.iter().collect()
}

fn main() {
    let instructions = get_input();

    let now = Instant::now();
    let one_dance = dance(&instructions, 1);
    let elapsed = now.elapsed();
    println!("The order of the programs after one dance is {}, took {}.{:09} seconds", one_dance, elapsed.as_secs(), elapsed.subsec_nanos());

    let hours = (elapsed * BILLION).as_secs() / 60 / 60;
    println!("That means fully running a billion iterations will take ~{} hours", hours);
    println!("Let's see if a shortcut can be found");

    let billion_dances = dance(&instructions, BILLION);
    println!("The order of the programs after a billion dances is {}", billion_dances);
}
