fn main() {
    println!("Hello, world!");

    another_function();


    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}


fn another_function() {
    println!("Another function.");
}
