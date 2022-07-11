
/**
rust 数据类型分两种: 标量类型 scalar 和 复合类型 compound
    标量类型: 单个值类型的统称(整数, 浮点数, 布尔值, 字符)
    复合类型: 元组(tuple), 数组(array)
rust 是一门静态类型语言, 编译时需要知道所有变量的类型，
*/
fn main() {
    // 标量类型
    let x: u32 = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    println!("The value of x is {}", x);
    let x = x * 2;
    println!("The value of x is {}", x);

    let y: f32 = 2.34;
    let y = y * 0.2;

    let mut _b: bool = true;
    _b = false;

    let _str: char = 'a';
    let _str = 'b';

    const MAX_POINTS: u32 = 100_000;
    println!("The const value of MAX_POINTS is {}", MAX_POINTS);

    // let guess = "42".parse().expect("Not a number");
    let spaces = "     ";
    println!("The value of spaces is {}.", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);


    // 复合类型
    tup_func();
    arr_func([56; 2]);
    demo1();
    let x: i32 = 34;

    println!("The function plus_one, parameter is {}, result is {}", x, plus_one(x));

    for i in 0..100 {
        if condition_match(i) {
            println!("The value {} matches the condition", i);
        }
    }

    println!("Test loop_func, result is {}", loop_func(10));

    println!("Test while_func, result is {}", while_func(19));

    println!("Test for_func, result is {}", for_func(12));

    let fibonacci_parameter: u32 = 10;
    println!("The fibonacci parameter is {}, result is {}", fibonacci_parameter, fibonacci(fibonacci_parameter));

}

/**
 tuple sample
 元组 示例
 */
fn tup_func() {
    //  元组: 将多个不同类型的值组合进一个复合类型中的复合类型
    //      拥有固定长度, 声明后不能增加或减少元素数量
    let tup: (u8, f32, bool, char) = (43, 3.4, false, ' ');
    println!("The sample of a tuple is {:?}", tup);
    println!("the sample of a tuple.0 is {},", tup.0);
    println!("the sample of a tuple.1 is {},", tup.1);
    println!("the sample of a tuple.3 is {},", tup.2);
    println!("the sample of a tuple.4 is {},", tup.3);
}

/**
 array sample
 数组 示例
 */
fn arr_func(arr: [u32; 2]) {
    println!("The parameters is {:?}.", arr);
    //  数组:固定长度, 声明后不能更改大小
    let array1: [u32; 6] = [2, 3, 4, 5, 6, 7];
    for x in array1 {
        println!("The array contains {}.", x);
    }
    let array2 = [13; 5];
    for x in array2 {
        println!("The array contains {}.", x);
    }
}

/**
 rust 中, 不以分号结尾的代码 都是表达式, 会有一个返回值
*/
fn demo1() {
    let x: u32 = 5;
    let y: i32 = {
        let x = 4;
        x - 1
    };

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}
/**
  rust 中,
    函数的返回值等于函数体最后一个表达式的值
    可以使用return关键字提前从函数中返回
 */
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn condition_match(x: i32) -> bool {
    x % 7 == 0 || x % 5 == 0
}

fn loop_func(x: i32) -> i32 {
    let mut sum = 0;
    loop {
        sum = sum + 1;
        // println!("test loop print {}.", x);
        if sum > x {
            break sum;
        }
    }
}

fn while_func(x: u32) -> u32 {
    let mut count = 0;
    while count < x {
        // println!("Test while print {}", count);
        count = count + 2;
    }
    count
}

fn for_func(x: u32) -> u32 {
    let mut result = 0;
    for i in (1..x).rev() {
        result = result + i * 2;
        // println!("tmp result: {}", result);
    }
    result
}

/**
 斐波那契数列算法
 */
fn fibonacci(x: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    if x == 1 || x == 2 {
        return 1;
    }
    fibonacci(x - 1) + fibonacci(x - 2)
}
