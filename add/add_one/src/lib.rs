use rand::{self, Rng};

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random_value(x: i32) -> i32 {
    let random_val: i32 = rand::thread_rng().gen();
    x + random_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_value() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}
