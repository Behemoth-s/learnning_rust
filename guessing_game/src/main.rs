use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜数字游戏!");
    // let apples = 5;
    // let mut bananas = 5;
    let mut guess = String::new();
    let sercet_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("输入你的猜测:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜测是:{}", guess);

        match guess.cmp(&sercet_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => print!("Too Big"),
            Ordering::Equal => {
                print!("Right");
                break;
            }
        }
    }
}
