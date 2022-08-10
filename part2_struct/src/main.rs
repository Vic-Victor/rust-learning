fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        username: String::from("cloud_j"),
        email: String::from("cloud_j@163.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.sign_in_count = 2;

    // let user2 = User {
    //     username: String::from("aaaa"),
    //     email: user1.email,
    //     sign_in_count: user1.sign_in_count + 1,
    //     active: user1.active,
    // };
    let user2 = User {
        username: String::from("aaaa"),
        sign_in_count: user1.sign_in_count + 1,
        ..user1
    };

    println!("user1 is {}, user2 is {}", user1.username, user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


}


// struct/structure 结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);

// 元组结构体
struct Point(i32, i32, i32);
