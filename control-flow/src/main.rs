fn main() {
    if_else_statement();
    for_loop();
    while_loop();
    loop_with_break();
    match_statement();
    match_with_enums();
    loop_with_continue();
    while_let_loop();
}

fn if_else_statement() {
    // if-else statement
    let numb = 10;
    if numb > 0 {
        println!("The number {} is positive", numb);
    } else if numb < 0 {
        println!("The number {} is negative", numb);
    } else {
        println!("The number is 0");
    }
    println!("-----------------------");
}

fn for_loop() {
    // for loop (inclusive of the number after '=')
    for i in 0..=5 {
        println!("Iteration: {}", i);
    }
    println!("-----------------------");
}

fn while_loop() {
    // while loop
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
    println!("-----------------------");
}

fn loop_with_break() {
    // infinite loop (with break)
    let mut count = 0;
    loop {
        println!("Loop count: {}", count);
        count += 1;

        if count == 5 {
            println!("Breaking the loop now");
            break;
        }
    }
    println!("-----------------------");
}

fn match_statement() {
    // match statement (similar to switch-case)
    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
    println!("-----------------------");
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn match_with_enums() {
    // match statement dependend on an enum
    let light = TrafficLight::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    }
    println!("-----------------------");
}

fn loop_with_continue() {
    // infinite loop with a continue
    for i in 0..=5 {
        if i == 3 {
            println!("Skipping 3");
            continue;
        }
        println!("Number: {}", i);
    }
    println!("-----------------------");
}

fn while_let_loop() {
    // while let loop
    let mut numbers = vec![1, 2, 3];

    while let Some(num) = numbers.pop() {
        println!("Popped: {}", num);
    }
    println!("-----------------------");
}