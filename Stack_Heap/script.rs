#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 2.7, y: 0.0}
}

pub fn stackHeap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; //Follow the pointer(*)
    println!("{}", p3.x);
}

fn main() {
    stackHeap();
}