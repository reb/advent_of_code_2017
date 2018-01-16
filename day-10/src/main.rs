extern crate knothash;

use knothash::hash;
use knothash::knothash;

fn get_input() -> (Vec<u8>, String) {
    const INPUT: &str = include_str!("input.txt");
    (convert(INPUT), INPUT.to_string())
}

fn convert(input: &str) -> Vec<u8> {
    input.lines().next().unwrap()
        .split(",")
        .map(|item| item.parse::<u8>().unwrap())
        .collect()
}

fn main() {
    let (lengths, input) = get_input();

    let single_hash = hash(lengths, 1);

    let product: u16 = single_hash.chunks(2)
        .next().unwrap().iter()
        .map(|item| *item as u16)
        .product();
    println!("Multiplying first two numbers in the list results in {}", product);

    let knot_hash = knothash(&input);

    println!("Knot Hash of the puzzle input: {}", knot_hash);
}


