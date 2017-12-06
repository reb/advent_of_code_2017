extern crate stringsort;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use stringsort::insertsort;

fn main() {
    let filename = "input.txt";

    let mut file = File::open(filename)
        .expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let unique_words_phrases = contents.lines()
        .filter(|line| {
            // check for uniqueness of all words on the line
            // by trying to insert all of them into a set
            let mut phrase = HashSet::new();
            return line.split(" ")
                .all(|word| phrase.insert(word));
        })
        .count();

    println!("There are {} pass-phrases with unique words", unique_words_phrases);

    let unique_anagram_phrases = contents.lines()
        .filter(|line| {
            // check for uniqueness of all permutations of 
            // words on the line by sorting the letters in
            // the word alphabetically and then trying to 
            // insert them all into a set
            let mut phrase = HashSet::new();
            return line.split(" ")
                .all(|word| phrase.insert(insertsort(word)));
        })
        .count();

    println!("There are {} pass-prhases with unique anagrams", unique_anagram_phrases);
}
