fn main() {
    let n = 40;
    let fib = fib(n);

    println!("fib {}: {}", n, fib);
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}
