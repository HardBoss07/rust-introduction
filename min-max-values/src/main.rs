fn main() {
    signedIntegers();
    unsignedIntegers();
    floats();
}

fn signedIntegers() {
    println!("Signed Integers:");
    println!("i8:   min = {}, max = {}", i8::MIN, i8::MAX);
    println!("i16:  min = {}, max = {}", i16::MIN, i16::MAX);
    println!("i32:  min = {}, max = {}", i32::MIN, i32::MAX);
    println!("i64:  min = {}, max = {}", i64::MIN, i64::MAX);
    println!("i128: min = {}, max = {}", i128::MIN, i128::MAX);
    println!("isize: min = {}, max = {}", isize::MIN, isize::MAX);
}

fn unsignedIntegers() {
    println!("Unsigned Integers:");
    println!("u8:   min = {}, max = {}", u8::MIN, u8::MAX);
    println!("u16:  min = {}, max = {}", u16::MIN, u16::MAX);
    println!("u32:  min = {}, max = {}", u32::MIN, u32::MAX);
    println!("u64:  min = {}, max = {}", u64::MIN, u64::MAX);
    println!("u128: min = {}, max = {}", u128::MIN, u128::MAX);
    println!("usize: min = {}, max = {}", usize::MIN, usize::MAX);
}

fn floats() {
    println!("Floating Point Numbers:");
    println!("f32:  min = {}, max = {}", f32::MIN, f32::MAX);
    println!("f64:  min = {}, max = {}", f64::MIN, f64::MAX);
}