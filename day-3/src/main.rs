use std::collections::HashMap;

/**
 * Calculate the position of a number on a spiralling grid
 *
 * Input:
 * 5        4       3 
 * 6        1       2
 * 7        8       9
 *
 * Expected output:
 * (-1, 1)  (0, 1)  (1, 1)
 * (-1, 0)  (0, 0)  (1, 0)
 * (-1, -1) (0, -1) (1, -1)
 */
fn grid_location(number: i32) -> (i32, i32) {

    // diagonal is the power
    let ring = (((number as f32).sqrt() - 1.0) / 2.0).ceil() as i32;
    // divide the ring in 4 arms of equal length
    let arm_length = 2 * ring;

    // calculate the marker position at (ring, -ring)
    let mut marker = (arm_length + 1) * (arm_length + 1);

    // arm underneath
    if number.ge(&(marker - arm_length)) {
        return (ring - (marker - number), -ring);
    }

    // arm to the left
    marker -= arm_length;
    if number.ge(&(marker - arm_length)) {
        return (-ring, ring - (marker - number));
    }

    // arm above
    marker -= arm_length;
    if number.ge(&(marker - arm_length)) {
        return (-ring + (marker - number), ring);
    }

    // arm to the right
    marker -= arm_length;
    return (ring, ring - (marker - number));
}


fn distance_to_access_port(number: i32) -> i32 {
    let (x, y) = grid_location(number);
    return x.abs() + y.abs();
}


fn get_neighbours_sum(grid: &HashMap<(i32, i32), i32>, (x, y): (i32, i32)) -> i32 {
    let mut sum = 0;

    for i in -1..2 {
        for j in -1..2 {
            if i.eq(&0) && j.eq(&0) {
                continue;
            }
            if let Some(value) = grid.get(&(x + i, y + j)) {
                sum += value;
            }
        }
    }

    return sum;
}

fn main() {
    // verify algorithm works
    assert_eq!(distance_to_access_port(1), 0);
    assert_eq!(distance_to_access_port(4), 1);
    assert_eq!(distance_to_access_port(7), 2);
    assert_eq!(distance_to_access_port(12), 3);
    assert_eq!(distance_to_access_port(23), 2);
    assert_eq!(distance_to_access_port(1024), 31);

    let input: i32 = 325489;
    let distance = distance_to_access_port(input);

    println!("Distance to access port for {} is: {}", input, distance);

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let (mut x, mut y) = (0, 0);
    let mut number = 1;
    grid.insert((x, y), number);

    loop {
        // if left and not up, go up
        if grid.contains_key(&(x - 1, y)) && !grid.contains_key(&(x, y +1)) {
            y += 1;
        }
        // if down, go left
        else if grid.contains_key(&(x, y - 1)) {
            x -= 1;
        }
        // if right, go down
        else if grid.contains_key(&(x + 1, y)) {
            y -= 1;
        }
        // go right
        else {
            x += 1;
        }

        number = get_neighbours_sum(&grid, (x, y));
        grid.insert((x, y), number);
        if number.ge(&input) {
            break;
        }
    }

    println!("First value larger than input written: {}", number);
}
