#[macro_use]
extern crate nom;

use nom::{digit};

named!(int<u32>,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8),
        std::str::FromStr::from_str
    )
);

named!(parse_line<(u32, u32)>,
    do_parse!(
        layer: int >>
        tag!(": ") >>
        depth: int >>
        (layer, depth)
    )
);

fn get_input() -> Vec<(u32, u32)> {
    const INPUT: &str = include_str!("input.txt");
    convert(INPUT)
}

fn convert(input: &str) -> Vec<(u32, u32)> {
    input.lines()
        .map(|line| parse_line(line.as_bytes()).to_result().ok().unwrap())
        .collect()
}

fn scanner_position(time: u32, depth: u32) -> u32 {
    let loop_length = depth * 2 - 2;
    let remaining_steps = time % loop_length;

    if remaining_steps < depth {
        return remaining_steps;
    } else {
        // takes (depth - 1) steps to get to the lowest position
        // all remaining steps then are walked up again
        let max_index = depth - 1;
        return max_index - (remaining_steps - max_index);
    }
}


fn main() {
    let firewall = get_input();

    let severity = &firewall.iter()
        .fold(0, |severity, &(layer, depth)| {
            if scanner_position(layer, depth) == 0 {
                return severity + layer * depth;
            }
            return severity;
        });

    println!("The severity of traversing the firewall without delay is {}", severity);

    let mut delay = 0;
    while !firewall.iter()
        .all(|&(layer, depth)| scanner_position(layer + delay, depth) != 0) {
        delay += 1;
    }

    println!("Delay by {} to prevent being caught by the firewall", delay);
}
