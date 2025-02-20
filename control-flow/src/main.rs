fn main() {
    // if-else statement
    let numb = 10;
    if numb > 0 {
        println!("The number {} is positive", numb);
    } else if numb < 0 {
        println!("The number {} is negative", numb);
    } else {
        println!("The number is 0");
    }

    // for loop (inclusive of the number after '=')
    for i in 0..=5 {
        println!("Iteration: {}", i);
    }
}
