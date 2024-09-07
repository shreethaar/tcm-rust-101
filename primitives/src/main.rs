fn main() {
    //Scalar types: int, float, boolean, char
    //Unsigned - never negative - u8, u16, u32, u64, u128, usize
    //Signed - can be negative and positive - default - i8, i16, i32, i64, i128, isize
    //Standard is i32 
    
    println!("Max size of a u32: {}", u32::MAX);
    println!("Max size of a u64: {}", u64::MAX);
    println!("Max size of a i32: {}", i32::MAX);
    println!("Max size of a i64: {}", i64::MAX);
    
    //int literals - decimals, binary
    //floats - f32, f64 (bits-precision)
    println!("Max size of a f32: {}", f32::MAX);
    println!("Max size of a f64: {}", f64::MAX);
    

    //boolean - true or false
    //represented - bool (if-else statement, conditional argument)
    
    //character - char (4 bytes)
    let c: char = 'A';
    println!("{}", c);

}
