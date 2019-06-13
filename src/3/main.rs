/*
 * Rustの構造体（メソッド）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let rect1 = Rectangle{width:3,height:4};
    println!("rect1: {:?}", rect1);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// implは分けて書ける
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}
impl Rectangle {
    fn can_hold(&self, target: &Rectangle) -> bool {
        target.width <= self.width && target.height <= self.height
    }
}
