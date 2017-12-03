use std::fs::File;
use std::io::prelude::*;


fn get_evenly_divisible_division(line: &str) -> i32 {
    let values = convert(line);

    for value in &values {
        for other in &values {
            if value != other && value % other == 0 {
                return value / other;
            }
        }
    }

    // should not be reached
    assert!(false);
    return 0;
}

fn get_min_max_difference(line: &str) -> i32 {
    let values = convert(line);

    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();
    return max - min;
}

fn convert(line: &str) -> Vec<i32> {
    return line.split("\t")
        .map(|item| item.to_string().parse::<i32>()
             .expect("Item could not be parsed to i32"))
        .collect::<Vec<i32>>();
}

fn main() {
    let filename = "input.txt";

    let mut file = File::open(filename)
        .expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let checksum_min_max : i32 = contents.lines()
        .map(|line| get_min_max_difference(line))
        .sum();

    println!("Checksum min max: {}", checksum_min_max);

    let checksum_evenly_divisible : i32 = contents.lines()
        .map(|line| get_evenly_divisible_division(line))
        .sum();

    println!("Checksum evenly divisible: {}", checksum_evenly_divisible);
}
