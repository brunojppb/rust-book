use std::fmt::Display;

struct MyAnn<'a> {
    value: &'a str,
}

impl Display for MyAnn<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyAnn<value: {}>", self.value)
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement!: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("I'm the longest");
    let b = String::from("I'm short");
    let my_ann = MyAnn { value: &b };

    let result = longest_with_announcement(&a, &b, my_ann);

    println!("Result: {}", result);
}
