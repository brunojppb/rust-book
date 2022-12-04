use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = `{}`", Rc::strong_count(&a));
    let _b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = `{}`", Rc::strong_count(&a));

    // wrapper to create a new scope for `c` so it gets cleaned up before main
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = `{}`", Rc::strong_count(&a));
    }

    println!(
        "count after c goes out of scope = `{}`",
        Rc::strong_count(&a)
    );
}
