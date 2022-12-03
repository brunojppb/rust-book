use std::fmt::Display;


fn longest_with_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T
) -> &'a str 
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
    let result = longest_with_announcement(&a, &b, "Look at this beauty!");
    
    println!("Result: {}", result);
}
