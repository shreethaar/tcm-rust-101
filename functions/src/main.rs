#![allow(unused)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // functions - mini programs, organized blocks of code
    who_am_i();
    add_one_hundred(100);
    add(7,7);
    println!("{}",multiply(5,5));
    let (added,multplied) = add_and_multiply(4,5);
    println("Added: {}",added);
    println("Multiplied: {}",multplied);
}

fn who_am_i() {
    let name = "Heath";
    let age = 33;
    println!("My name is {} and I am {} years old.",name,age);
}

fn add_one_hundred(num:i32) {
    println!("{}", num+100);
}

fn add(x:i32,y:i32) {
    println("{}",x+y);
}

fn multiply(x:i32,y:i32) -> i32 {
    x*y
}

fn add_and_multiply(x:i32,y:i32) -> (i32,i32) {
    (x+y,x*y)
}

