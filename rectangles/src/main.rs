fn main() {
    let width = 1920;
    let height = 1080;

    println!("The area of the rectangle is {} square pixels", area(width, height));

    let rect = (1440, 1240);
    println!("The area1 of the rectangle is {} square pixels", area1(rect));

    let rectangle = Rectangle {width: 900, height: 480};
    // println!("The rectangle is {:#?}", rectangle);
    // println!("The area2 of the rectangle is {} square pixels", area2(rectangle))
    println!("The area3 of the rectangle is {} square pixels", rectangle.area3());

    let otherRectangle = Rectangle {width:1000, height: 500};
    println!("Can hold = {}", rectangle.can_hold(&otherRectangle));
    println!("Can hold = {}", otherRectangle.can_hold(&rectangle));

}

// 通过像素为单位的长方形的宽度和高度, 并计算出长方形的面积
fn area(width: u32, height: u32) -> u32 {
    width * height
}


fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 结构体: 长方形的宽和高
// #[derive(Debug)] 显式的选择结构体的打印出调试信息的功能
// 每个结构体都允许有多个 impl 模块
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle impl 模块1
impl Rectangle {
    fn area3(&self) -> u32 {
        self.width * self.height
    }
}

// Rectangle impl 模块2
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width && self.height >= other.height) || (self.width >= other.height && self.height >= other.width)
    }
}