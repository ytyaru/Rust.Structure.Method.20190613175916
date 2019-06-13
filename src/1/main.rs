/*
 * Rustの構造体（メソッド）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let rect1 = Rectangle { width: 3, height: 4 };
    let rect2 = Rectangle { width: 30, height: 40 };
    println!("rect1 > rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 > rect1: {}", rect2.can_hold(&rect1));
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
    fn can_hold(&self, target: &Rectangle) -> bool {
        target.width <= self.width && target.height <= self.height
//        if target.width <= self.width && target.height <= self.height { true }
//        else { false }
    }
}
