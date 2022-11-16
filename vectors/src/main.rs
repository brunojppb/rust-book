fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 10;
    }

    println!("Vector value is {:?}", v);
}
