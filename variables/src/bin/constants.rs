fn main() {
    let x = 5;
//This program first binds x to a value of 5.
//Then it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6.
//Then, within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12. When that scope is over, the inner shadowing ends and x returns to being 6

    let x = x + 1;
    {
        let x = x + 2;
        println!(" The value of x is : {x}");
    }

    println!("The value of x here is: {x}");

    let spaces = "   ";
    //this will work as shadowing allow us to perform some operations without changing the name
    let spaces = spaces.len();

    println!("Value of spaces is: {spaces}");

    let mut length = "  ";

    // this will give an compile type error saying that we cant change the type of variable assigned as mut
    length = length.len();
}
