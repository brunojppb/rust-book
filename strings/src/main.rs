fn main() {
    let hi = String::from("नमस्ते");

    println!("Iterating over characters (Unicode Scalar values)");
    for c in hi.chars() {
        println!("Char is {}", c);
    }

    println!("\nIterating over bytes");
    for c in hi.bytes() {
        println!("Byte is {}", c);
    }

    println!("\nIterating over Grapheme clusters?");
    // for that we need to use a crate
    // See: https://stackoverflow.com/questions/58770462/how-to-iterate-over-unicode-grapheme-clusters-in-rust
    // See: https://crates.io/crates/unicode-segmentation
}
