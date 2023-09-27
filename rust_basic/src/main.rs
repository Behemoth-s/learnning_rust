fn F2C(f: f64) -> f64 {
    // fahrenheit to celsius
    (f - 32.0) * 5.0 / 9.0
}
fn C2F(c: f64) -> f64 {
    // celsius to fahrenheit
    c * 9.0 / 5.0 + 32.0
}
fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
fn main() {
    let f = 100.0;
    let c = F2C(f);
    println!("f = {f} to c =  {c}");
    let c = 100.0;
    let f = C2F(c);
    println!("c = {c} to f = {f}");
    let f = fib(3);
    println!("fib 3= {f}");
    let f = fib(5);
    println!("fib 5= {f}");
    println!("Hello, world!");
}
