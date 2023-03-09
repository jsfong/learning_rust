#![allow(dead_code)]

use std::mem;

struct Point
{
    x: f64, //8 bytes
    y: f64, //8 bytes
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}


pub fn stack_and_heap() {
    //Stack allocation
    let p1 = origin();

    //Heap allocation
    let p2 = Box::new(origin()); //Store address only

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; // unboxing follow the reference
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
}
