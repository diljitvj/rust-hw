#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}


fn main(){
    let o = Point{ x: 0.0, y: 0.0};
    // Allocating a value on to the heap
    let a = Box::new(Point{ x: 0.0, y: 0.0});

    println!("Size of o = {} bytes", mem::size_of_val(&o));
    println!("Size of a = {} bytes", mem::size_of_val(&a));
    // Allocating back to the stack
    let b = *a;

    println!("Size of b = {} bytes", mem::size_of_val(&b));

}