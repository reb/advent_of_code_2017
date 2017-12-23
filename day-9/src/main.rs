use std::str::Chars;

fn get_input() -> &'static str {
    include_str!("input.txt").trim()
}

fn garbage(iterator: &mut Chars) -> i32 {
    let mut size = 0;
    while let Some(character) = iterator.next() {
        match character {
            '>' => return size,
            '!' => {
                    iterator.next();
                },
            _   => size += 1,
        };
    }
    panic!("Garbage did not end properly");
}

fn group(iterator: &mut Chars, level: i32) -> (i32, i32) {
    let mut score = 0;
    let mut garbage_size = 0;

    while let Some(character) = iterator.next() {
        match character {
            '{' => { 
                    let (group_score, group_garbage_size) = group(iterator, level + 1);
                    score += group_score;
                    garbage_size += group_garbage_size;
                },
            '<' => garbage_size += garbage(iterator),
            ',' => continue,
            '}' => return (score + level, garbage_size),
            _   => panic!("Found an illegal character outside of garbage"),
        }
    }
    (score, garbage_size)
}


fn main() {
    let input = get_input();

    let (score, garbage) = group(&mut input.chars(), 0);
    println!("Score of the input is {}", score);
    println!("The amount of garbage is {}", garbage);
}
