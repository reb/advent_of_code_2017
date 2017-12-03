use std::fs::File;
use std::io::prelude::*;

/*
 * Shift a string by the given offset and loop it around
 */
fn shift(input: String, offset: usize) -> String {
    let mut input_shifted = input[offset..].to_string();
    input_shifted.push_str(&input[..offset].to_string());
    return input_shifted;
}

fn sum_matched_numbers(input: &str, input_shifted: &str) -> i32 {
    return input.chars()
        // combine both strings in one iterator
        .zip(input_shifted.chars())
        // filter out non-matches
        .filter(|&(character, shifted)| character == shifted)
        // convert character tuple to integer
        .map(|(character, _)| character.to_string().parse::<i32>()
             .expect("Character could not be parsed to i32")
             )
        // fold to sum
        .fold(0, |sum, number| sum + number);
}

fn main() {
    let filename = "input.txt";

    let mut file = File::open(filename)
        .expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    // clean new lines
    let input = contents.lines().next().unwrap();

    // shift the input by 1
    let input_shifted_1 = shift(input.to_string(), 1);

    let captcha_compare_next = sum_matched_numbers(&input, &input_shifted_1);
    println!("The captcha result with comparing next number: {}", captcha_compare_next);

    // shift the input by half the length
    let input_shifted_half_length = shift(input.to_string(), input.len() / 2);

    let captcha_compare_half_length = sum_matched_numbers(&input, &input_shifted_half_length);
    println!("The captcha result with comparing the number half the length further: {}", captcha_compare_half_length);
}
