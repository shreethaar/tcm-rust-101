#![allow(unused)]

use std::io;
use rand::Rng;
fn main() {
    let x = 10;
    let y = 3;
    let x_float = x as f64;
    let y_float = y as f64;

    println!("{} + {} = {}",x,y,x+y);
    println!("{} - {} = {}",x,y,x-y);
    println!("{} x {} = {}",x,y,x*y);
    println!("{} / {} = {}",x,y,x_float/y_float);
    println!("{} % {} = {}",x,y,x%y); //remainder
    println!("{}^{} = {}",x,y,i32::pow(x,y.try_into().unwrap()));

    let z = rand::thread_rng().gen_range(1..101);
    println!("{}",z);

}
