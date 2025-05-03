fn main() {
    let x = 5;
    println!("The value of x is : {x}");

    //This will give an error cannot assign twice to immutable variable because we tried to assign a second value to the immutable variable x, to fix this issue either we can make it

    x = 6;
    println!("The value of x is : {x}");
}
