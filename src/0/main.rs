/*
 * Rustの構造体（メソッド）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let rect = Rectangle { width: 3, height: 4 };
    println!("w: {} h: {} area: {}", rect.width, rect.height, rect.area());
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}
