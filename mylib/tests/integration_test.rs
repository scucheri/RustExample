extern crate mylib;
use mylib::xiaoyu;
mod common;


#[cfg(test)]
#[test]
fn xiaoyu_add_two_num() {
    assert_eq!(4, xiaoyu::add(2, 2));
}

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, xiaoyu::add(2, 2));
}