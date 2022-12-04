use smart_pointers::List::{Cons, Nil};
use smart_pointers::MyBox;

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Cons list: {:?}", list);

    // playing with my own smart pointer
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref works on MyBox with other times that implement Deref
    let name = MyBox::new(String::from("Bruno"));
    hello(&name);
}
