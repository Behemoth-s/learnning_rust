fn main() {
    let mut x = 5;
    println!("Value of x is {x}");
    x = 6;
    println!("Value of x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let y = 5;
    let y = y + 1;
    {
        println!("Value of y is {y}");
        let y = y + 2;
        println!("Value of y is {y}");
    }
    print!("Value of y is {y}");
}
