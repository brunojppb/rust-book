#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(scale * 10),
        height: 20,
    };

    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
