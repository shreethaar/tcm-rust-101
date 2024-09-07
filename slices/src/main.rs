#![allow(unused)]

use std::io;

fn main() {
    let mut arr:[i32,5] = [1,2,3,4,5];
    // element 2 and 3
    let slice: &mut [i32] = &mut arr[1..3];
    // grepping 1st element and ending before 3rd element
    println!("{:?}",slice);
    let slice2 = &mut arr[3..5];
    println!("{:?}",slice2);
    
    slice[0] = 6;
    slice[1] = 7;
    println!("{:?}",arr)
}
