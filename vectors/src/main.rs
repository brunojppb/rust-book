fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 10;
    }

    println!("Vector value is {:?}", v);

    #[derive(Debug)]
    enum MyBox {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let values = vec![
        MyBox::Int(10),
        MyBox::Float(3.14),
        MyBox::Text(String::from("hi there!")),
    ];

    println!("Values for my custom vec is {:?}", values);
}
