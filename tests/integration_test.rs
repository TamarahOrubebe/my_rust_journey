use my_rust_journey;

mod common;

#[test]
fn it_adds_two_numbers() {
    common::setup();
    assert_eq!(4, my_rust_journey::add(2, 2));
}