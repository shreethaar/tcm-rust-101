#![allow(unused)]

use std::io;

fn main() {
    /* Build a simple calculator that takes two user inputs
       then calculates the addition, subtraction, multiplication, and division
       of those two inputs.
    */

    println!("Give me a value for x");
    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x);

    let x: i32 = input_x.trim().parse().expect("Entry was not an integer!");
    let float_x = x as f64;

    println!("Give me a value for y");
    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y);

    let y: i32 = input_y.trim().parse().expect("Entry was not an integer!");
    let float_y = y as f64;

    println!("The result of {} + {} is {}", x, y, x+y);
    println!("The result of {} - {} is {}", x, y, x-y);
    println!("The result of {} * {} is {}", x, y, x*y);
    println!("The result of {} / {} is {}", x, y, float_x/float_y);
}
