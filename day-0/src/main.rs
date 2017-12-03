use std::fs::File;
use std::io::prelude::*;

fn taxi_distance_from_source((x, y): (i32, i32)) -> i32 {
    return x.abs() + y.abs();
}

fn taxi_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    return (x1-x2).abs() + (y1-y2).abs();
}

fn main() {
    let mut file = File::open("elvish_cheat_codes.txt")
        .expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let actions = contents.split(", ");

    let (mut x, mut y) = (0, 0);
    let mut a_locations = Vec::new();
    let mut b_locations = Vec::new();
    for action in actions {
        match action {
            "Up"    => {y += 1;}
            "Down"  => {y -= 1;}
            "Right" => {x += 1;}
            "Left"  => {x -= 1;}
            "A"     => {a_locations.push((x,y))}
            "B"     => {b_locations.push((x,y))}
            "Start" => {break;}
            _ => { ; }
        }
    }

    let mut locations = a_locations.clone();
    locations.extend(b_locations.clone());

    let max_from_source = locations.iter()
        .map(|location| taxi_distance_from_source(*location))
        .max()
        .expect("No distance found from source");

    println!("maximum distance from source: {:?}", max_from_source);

    let max_distance_pair = a_locations.iter()
        .map(|a_location| b_locations.iter()
            .map(|b_location| taxi_distance(*a_location, *b_location))
            .max()
            .expect(&format!("No distances found for A location: {:?}", a_location)))
        .max()
        .expect("No distances found for any location");

    println!("maximum distance pair: {:?}", max_distance_pair);
}
