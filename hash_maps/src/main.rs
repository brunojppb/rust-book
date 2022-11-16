use std::collections::HashMap;

use crate::tasks::numbers::numbers_task;
mod tasks;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let numbers = vec![1, 2, 2, 3, 3, 2, 5, 5, 5, 5];
    println!("Median and mode: {:?}", numbers_task(&numbers));
}
