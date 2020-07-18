fn main() {
    let f = fib(11);
    println!("{}", f);
}

fn fib(n: i32) -> i32 {
    match n {
        0..=1 => n,
        _ => fib(n - 1) + fib(n - 2)
    }
}