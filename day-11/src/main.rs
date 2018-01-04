use std::cmp;

fn get_input() -> Vec<String> {
    const INPUT: &str = include_str!("input.txt");
    convert(INPUT)
}

fn convert(input: &str) -> Vec<String> {
    input.lines().next().unwrap()
        .split(",")
        .map(|item| item.to_string())
        .collect()
}

fn distance_from_0((x, y, z): (i32, i32, i32)) -> i32 {
    (x.abs() + y.abs() + z.abs()) / 2
}

fn main() {
    /*
     * Using a cube hexagon co-ordinate system
     *
     *    \ n  /
     *  nw +--+ ne
     *    / y  \
     *  -+    x +-
     *    \ z  /
     *  sw +--+ se
     *    / s  \
     *
     * this means going 'nw' from (0, 0, 0) will be x-1 and y+1 resulting in (-1, 1, 0)
     * going 's' will be z+1 and y-1
     */

    let steps = get_input();
    let ((x, y, z), max_distance) = steps.iter()
        .fold(((0, 0, 0), 0), |((x, y, z), max), step| {
            let location = match step.as_ref() {
                "n"     => {(x, y+1, z-1)}
                "nw"    => {(x-1, y+1, z)}
                "sw"    => {(x-1, y, z+1)}
                "s"     => {(x, y-1, z+1)}
                "se"    => {(x+1, y-1, z)}
                "ne"    => {(x+1, y, z-1)}
                _       => {(x, y, z)}
            };
            (location, cmp::max(max, distance_from_0(location)))
        });

    let distance = distance_from_0((x, y, z));

    println!("End location is ({}, {}, {}) and that's {} steps away", x, y, z, distance);
    println!("The furthest ever gotten is {}", max_distance);

}
