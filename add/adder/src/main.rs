use add_one;

fn main() {
    let result = add_one::add_one(1);
    println!("add one result: {}", result);

    let random_result = add_one::add_random_value(10);
    println!("Random result: {}", random_result);
}
