fn main() {
    println!("Hello, world!");

    println!("The result is:{}", add(10, 20));
    println!("The result is:{}", subtract(20, 10));
    println!("The result is:{}", multiply(10, 10));
    println!("The result is: {}", divide(10, 5));

    classify_number(25);
}

fn add(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}
fn subtract(x: i32, y: i32) -> i32 {
    let z = x - y;
    return z;
}

fn multiply(p: i32, q: i32) -> i32 {
    let r = p * q;
    return r;
}

fn divide(a: i32, b: i32) -> i32 {
    let c = a / b;
    return c;
}

fn classify_number(n:i32){
  match n {
    0 => println!("{} is zero", n),
    1..=i32::MAX => println!("{} is positive", n),
    _ => println!("{} is negative", n),
  }
}