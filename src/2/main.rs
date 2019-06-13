/*
 * Rustの構造体（メソッド）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let rect1 = Rectangle::square(5);
    println!("rect1: {:?}", rect1);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // 第一引数がselfでないものは「関連関数」である
    fn square(size: u32) -> Rectangle { Rectangle { width: size, height: size } }
    fn area(&self) -> u32 { self.width * self.height }
    fn can_hold(&self, target: &Rectangle) -> bool {
        target.width <= self.width && target.height <= self.height
    }
}
