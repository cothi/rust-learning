fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // not working
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // Array Type

    let _a = [1, 2, 3, 4, 5];
    let _month = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] = {}", a[0]);
}
