fn main() {
    let condition = true;

    // ternary expression
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
