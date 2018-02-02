fn main() {
    let steps = 356;

    // for the first part execute a full simulation of the circular buffer
    let mut buffer = vec!(0);
    let mut index = 0;

    for i in 1..2017+1 {
        index = ((index + steps) % buffer.len()) + 1;
        buffer.insert(index, i);
    }

    let number_after_last = buffer.get(index + 1).unwrap();
    println!("The number in the complete circular buffer after 2017 is {}", number_after_last);


    // For the second part a full simulation isn't feasible, but because the 
    // value is always inserted after the given index it is know that 0 will 
    // stay at the first position in the buffer.
    //
    // With knowning the buffer size (which will be the same as the iteration 
    // number) the new index insertion position can be calculated, then only
    // write down a number if it's inserted at index 1
    let mut number_after_0 = 0;
    index = 0;
    for i in 1..50000000+1 {
        let buffer_size = i;
        index = ((index + steps) % buffer_size) + 1;

        if index == 1 {
            number_after_0 = i;
        }
    }

    println!("The number in the circular buffer after 0 at 50.000.000 insertions is {}", number_after_0);
}
