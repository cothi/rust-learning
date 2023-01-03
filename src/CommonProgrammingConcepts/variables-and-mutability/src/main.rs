fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // this is not compile
    // x = 6;
    //println!("The value of x is: {x}");

    let mut y = 5;

    println!("The value of y is: {y}");

    // this is compile
    y = 6;
    println!("The value of y is: {y}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The scope value of x is: {x}");
    }
    println!("The outer scope value of x is: {x}");

    // let spaces = "  ";
    // this is not compile
    // let mut spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");
}
