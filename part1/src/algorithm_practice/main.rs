fn main() {
    println!("Hello, world!");
    let x: u32 = 24;
    println!(
        "The parameter is {}, result of `fibonacci` is: {}",
        x,
        fibonacci(x)
    );

    println!("The first_word result is: {}", first_word("  askjdakw"));

    // Slices 是String 中一部分值的引用
    // start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的range.
    // 如果需要包含end, 可以使用..= 而不是..
    // 即:
    //      .. 是前闭后开区间
    //      ..= 是前闭后闭区间
    let s: &String = String::from("Hello World!");

    let hello = &s[0..5]; // "hello"
    let world = &s[6..11]; // "world"
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
    fibonacci(x - 1) + fibonacci(x - 2)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
