use smart_pointers::List::{Cons, Nil};
use smart_pointers::MyBox;

fn hello(name: &str) {
    println!("Hello {}", name);
}
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
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

    // Drop trait
    let _c = CustomSmartPointer {
        data: String::from("this is cool and is going to the bin..."),
    };

    let _d = CustomSmartPointer {
        data: String::from("this is also going to the bin..."),
    };

    println!("Smart pointers created...");

    let f = CustomSmartPointer {
        data: String::from("forced drop..."),
    };

    println!("SmartPointer to be force-dropped created...");

    drop(f);
}
