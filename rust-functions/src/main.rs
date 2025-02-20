fn main() {
    let mut a = 10;
    let mut b = 20;
    println!("Variable a = {}\nVariable b = {}", a, b);

    println!("Add = {}", addTwoNumbers(a, b));
    println!("Subtract = {}", subtractTwoNumbers(a, b));

    a = addTwoNumbers(a, b);
    b = subtractTwoNumbers(a, b);
    println!("Variable a = {}\nVariable b = {}", a, b);

    println!("Add = {}", addTwoNumbers(a, b));
    println!("Subtract = {}", subtractTwoNumbers(a, b));
}

fn addTwoNumbers(a: i32, b: i32) -> i32 {
    return a + b;
}

fn subtractTwoNumbers(a: i32, b: i32) -> i32 {
    return a - b;
}
