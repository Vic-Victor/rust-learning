fn main() {
    println!("Hello, world!");
    let x: u32 = 24;
    println!("The parameter is {}, result of `fibonacci` is: {}", x, fibonacci(x));
}

/**
 a fibonacci function sample
 斐波那契数列函数
 */
fn fibonacci(x: u32) -> usize {
    if x == 0 {
        return 0;
    }
    if x == 1 || x == 2 {
        return 1;
    }
    fibonacci(x - 1) + fibonacci(x -2)
}