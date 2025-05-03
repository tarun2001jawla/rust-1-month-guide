fn square(num: i32) -> i32 {
    num * num
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn main() {
    let number = 8;
    println!("The square of {} is {}", number, square(number));
    
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // Expression block
    let doubled = {
        let base = 10;
        base * 2
    };
    println!("Doubled: {}", doubled);
}
