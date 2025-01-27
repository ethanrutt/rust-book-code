fn fib(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn print_fib() {
    let res = fib(0);
    println!("fib(0): {res}");
    let res = fib(1);
    println!("fib(1): {res}");
    let res = fib(2);
    println!("fib(2): {res}");
    let res = fib(3);
    println!("fib(3): {res}");
    let res = fib(4);
    println!("fib(4): {res}");
    let res = fib(5);
    println!("fib(5): {res}");
    let res = fib(6);
    println!("fib(6): {res}");
    let res = fib(7);
    println!("fib(7): {res}");
    let res = fib(8);
    println!("fib(8): {res}");
    let res = fib(9);
    println!("fib(9): {res}");
}

fn f_to_c(a: f64) -> f64 {
    (a - 32f64) * (5f64 / 9f64)
}

fn c_to_f(a: f64) -> f64 {
    (a * (9f64 / 5f64)) + 32f64
}

fn print_temp_conversions() {
    let res = c_to_f(32f64);
    println!("c_to_f(32f64): {res}");
    let res = f_to_c(32f64);
    println!("f_to_c(32f64): {res}");
}

fn main() {
    print_fib();
    print_temp_conversions();
}
