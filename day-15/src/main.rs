fn generate(start: u64, multiplication_factor: u64, modulo_criteria: u64) -> u64 {
    let mut new = start;
    loop {
        new = (new * multiplication_factor) % 2147483647;
        if new % modulo_criteria == 0 {
            return new
        }
    }
}

fn judge_generators(a_start: u64, b_start: u64, a_factor: u64, b_factor: u64, a_criteria: u64, b_criteria: u64, pairs: u32) -> u32 {
    let mut a = a_start;
    let mut b = b_start;

    let mut matches = 0;
    for _ in 0..pairs {
        a = generate(a, a_factor, a_criteria);
        b = generate(b, b_factor, b_criteria);

        if a as u16 == b as u16 {
            matches += 1;
        }
    }

    matches
}


fn main() {
    let a_start = 783;
    let b_start = 325;

    let a_factor = 16807;
    let b_factor = 48271;

    let simple_pairs = 40000000;

    let simple_matches = judge_generators(a_start, b_start, a_factor, b_factor, 1, 1, simple_pairs);
    println!("Found {} matches after {} pairs, using simple generator logic", simple_matches, simple_pairs);

    let a_criteria = 4;
    let b_criteria = 8;
    let complex_pairs = 5000000;
    let complex_matches = judge_generators(a_start, b_start, a_factor, b_factor, a_criteria, b_criteria, complex_pairs);
    println!("Found {} matches after {} pairs, using complex generator logic", complex_matches, complex_pairs);
}
