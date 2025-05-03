fn main() {
    println!("Rust Data Types Practice");

    // ----------- Scalar Types -----------
    //1. Integer Types
    let a: i64 = -42;
    let b: u8 = 200;

    //2. Floating-point types
    let c: f64 = 3.14;
    let d: f32 = 2.71;

    //3. Boolean types
    let is_rust_fun: bool = false;

    //4. Character type
    let emoji: char = '🚀';

    println!("\n📦 Scalar Types:");
    println!("a (i64): {}", a);
    println!("b (u8): {}", b);
    println!("c (f32): {}", c);
    println!("d (f64): {}", d);
    println!("is_rust_fun (bool): {}", is_rust_fun);
    println!("emoji (char): {}", emoji);

    // ----------- Compound Types -----------

    //1. Tuple

    let tup: (i32, f64, char) = (500, 6.4, 'T');

    let (x, y, z) = tup;

    println!("\n Tuple:");
    println! ("tup.0 = {}", tup.0);
    println!("Destructured tuple is: x ={}, y ={}, z= {}", x, y, z);

    //2. Array

    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("\n Array:");
    println!("arr[0]= {}", arr[0]);
}
