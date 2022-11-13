fn main() {
    // let s = "Hi there!";

    let mut s2 = String::from("what?");

    expand_str(&mut s2);

    println!("S2 now is {s2}");
}

fn expand_str(value: &mut String) {
    value.push_str("Expanded! boom!");
}
