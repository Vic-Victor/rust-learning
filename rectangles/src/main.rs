fn main() {
    let width = 1920;
    let height = 1080;

    println!("The area of the rectangle is {} square pixels", area(width, height));

    let rect = (1440, 1240);
    println!("The area1 of the rectangle is {} square pixels", area1(rect))

}

// 通过像素为单位的长方形的宽度和高度, 并计算出长方形的面积
fn area(width: u32, height: u32) -> u32 {
    width * height
}


fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}