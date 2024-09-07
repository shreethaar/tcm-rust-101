#![allow(unused)]

use std::io;

fn main() {
    println!("Who goes there?");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    let enter = "You may now enter.";
    println!("Hello there {}. {}", name.trim_end(), enter);
}
