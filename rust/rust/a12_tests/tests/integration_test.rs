// Rust will compile each file under ./tests as an individual crate.
// Unlike sub-module tests, this test can only use the external API of the library: `pub` functions.


use a12_tests;

pub mod common;
// use common; // use a submodule

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, a12_tests::add(2, 2));
}