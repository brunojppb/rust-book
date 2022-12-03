use adder;

mod common;

#[test]
fn adds_two() {
    common::setup();

    assert_eq!(2, adder::add(1, 1));
}
