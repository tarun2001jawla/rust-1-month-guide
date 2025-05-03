fn main() {
    // if-else

    let age = 20;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are underage.");
    }
    // loop

    let mut attempts = 0;
    loop {
        println!("Attempt: {}", attempts);
        attempts += 1;
        if attempts == 3 {
            println!("Max attempts reached.");
            break;
        }
    }

    // while loop

    let mut num = 0;
    while num > 0 {
        println!("Countdown: {}", num);
        num -= 1;
    }

    // for loop

    for i in 1..=5 {
        println!("For loop iteration: {}", i);
    }

    // match

    let temp = 30;
    match temp {
        t if t > 35 => println!("It's too hot"),
        25..=35 => println!("Perfect weather"),
        _ => println!("Its too cold here."),
    }
}
