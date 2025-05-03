
fn main() {
    println!("Integer Overflow Demo in Rust");
    println!("--------------------------------");

    let max_value: u8 = 255;
    let increment: u8 = 0;

    println!("\n Trying to add {} + {}", max_value, increment);

    //1. Wrapping Add

    let wrap = max_value.wrapping_add(increment);
    println!("Wrapping add: {}", wrap); //Output will be 0

    //2. Checked Add
    match max_value.checked_add(increment) {
        Some(val) => println!("checked_add: {}", val),
        None => println!("checked_add: Overflow Detected"),
    }

    //3. Overflowing Add
    let (val, did_overflow) = max_value.overflowing_add(increment);
    println!("overflowing_add: {}, overflowed: {}", val, did_overflow);

    //4. Saturating Add
    let sat = max_value.saturating_add(increment);
    println!("saturating_add: {}", sat);
}
