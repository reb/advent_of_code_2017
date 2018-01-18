extern crate knothash;

use knothash::knothash;
use std::cmp::max;
use std::collections::HashMap;

fn count_connected_regions(grid: &Vec<String>) -> u32 {
    let mut previous_line = vec![0; 128];
    let mut count = 0;
    let mut next_color = 1;
    
    let mut merged = HashMap::new();


    for line in grid.iter() {
        let mut current_color = 0;
        for (i, item) in line.chars().enumerate() {
            if item == '0' {
                // nothing here, empty current_color
                current_color = 0;
            } else {
                let mut previous_line_color = previous_line[i];
                while let Some(&translated_color) = merged.get(&previous_line_color) {
                    previous_line_color = translated_color;
                }
                if current_color != 0 && previous_line_color != 0 && current_color != previous_line_color {
                    // two regions are merging:
                    //  - there is a color on the left, because current_color is not 0
                    //  - there is a color above, because the previous_line is not 0
                    //  - these colors are not the same
                    count -= 1;
                    merged.insert(previous_line_color, current_color);
                } else {
                    // we need to paint this area with something
                    current_color = max(current_color, previous_line_color);
                    if current_color == 0 {
                        // no color in the surrounding area, pick a new one
                        count += 1;
                        current_color = next_color;
                        next_color += 1;
                    }
                }
            }

            previous_line[i] = current_color;
        }
    }
    return count;
}


fn main() {
    let input = String::from("uugsqrei");

    let hashes: Vec<String> = (0..128)
        .map(|i| knothash(&format!("{}-{}", input, i)))
        .collect();

    let numbers: Vec<(u64, u64)> = hashes.iter()
        .map(|hash| {
            // if using nightly, all this would be:
            // u128::from_str_radix(&hash, 16).unwrap();
            let start: String = hash.chars().take(16).collect();
            let end: String = hash.chars().skip(16).take(16).collect();

            let start_number = u64::from_str_radix(&start, 16).unwrap();
            let end_number = u64::from_str_radix(&end, 16).unwrap();

            return (start_number, end_number);
        })
        .collect();

    let used: u32 = numbers.iter()
        .map(|&(start, end)| start.count_ones() + end.count_ones())
        .sum();


    println!("The number of squares used is {}", used);

    let grid = numbers.iter()
        .map(|&(start, end)| format!("{:064b}{:064b}", start, end))
        .collect();

    let regions = count_connected_regions(&grid);

    println!("The number of different regions present is {}", regions);
}
