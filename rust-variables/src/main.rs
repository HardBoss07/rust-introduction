fn main() {
    // immutable = value once assigned cannot be changed
    let x = 5;
    println!("the immutable variable x = {}", x);

    // mutable = value can be modified later
    let mut y = 5;
    println!("the mutable variable y = {}", y);
    y += 10;
    println!("the mutable variable y = {}", y);

    // data types
    let a: i32 = 10;
    println!("i32 a = {}", a);
    let b: f64 = 3.14;
    println!("f64 a = {}", b);
    let c: bool = true;
    println!("bool c = {}", c);
    let d: char = 'R';
    println!("char d = {}", d);
    let e: String = String::from("Matteo");
    println!("String = {}", e);

    // tuple = fixed length list with different data types
    let tuple: (String, u8, f64) = (String::from("Matteo"), 25, 6.9);
    println!("tuple = {:?}", tuple);
    println!("deconstructed tuple:\nName = {}\nAge = {}\nScore = {}", tuple.0, tuple.1, tuple.2);

    // array = fixed length list with same data types
    let array: [i32; 4] = [10, 20, 30, 40];
    println!("full array = {:?}", array);
    println!("array value 0 = {}", array[0]);
    println!("array value 1 = {}", array[1]);
    println!("array value 2 = {}", array[2]);
    println!("array value 3 = {}", array[3]);
    for num in array.iter() {
        println!("array value = {}", num);
    }
}
