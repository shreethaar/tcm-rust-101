#![allow(unused)]

fn main() {
    //strings - there are several types
    //likely only use two: String and &str

    //str - string slice, &str - borrowed string slice - cannot be modified
    //String - data can be modified
    //&str - essentially a subset of a String 
    
    let name = "Heath".tostring();
    let name = String::from("Heath");

    let mut name = String::new();
    name.push_str("Heath");
    name.push_str(" test");
    println!("{}",name);

    //escaping
    println!("Hello, world!");
    println!("Hello, \
    world!");
    println!("He said, \"All is fair in love and war\"");
    println!("{}",concat!("Hello","World"));
    println!("{}",concnat!("Hello ","world!"));

}
