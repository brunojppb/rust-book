use std::collections::HashMap;

// Given a list of integers, use a vector and return the
// median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will
// be helpful here) of the list.
pub fn numbers_task(list: &Vec<i32>) -> (i32, i32) {
    let mut sorted = list.clone();
    sorted.sort();
    let median = sorted[sorted.len() / 2];
    let mut mode_count = HashMap::new();
    for v in list {
        let count = mode_count.entry(v).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut high_key = 0;

    for (key, value) in &mode_count {
        if *value > mode {
            high_key = **key;
            mode = *value;
        }
    }

    (median, high_key)
}
