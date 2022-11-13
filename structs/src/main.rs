#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(scale * 10),
        height: 20,
    };

    println!("Rect area is {}", rect.area());
}
