//constants
const NUMBER: i32 = 16;

fn main() {
    //variables are immutable
    let hello: &str = "Hello, world!";
    println!("{}",hello);

    //to make mutable
    let mut hello2: &str = "Hello2";
    println!("{}", hello2);
    hello2="Hello again!";
    println!("{}", hello2);

    //num
    let x: i32 = 5;
    let y: i32 = 6;
    println!("Math in rust: {} + {} = {}",x,y,x+y);

    //constants
    //const NUMBER: i32 = 17; 
    println!("{}",NUMBER);
    // constant can be place outside of the function
    
    let x: i32 = 1;
    {
        let y: i32 = 2;
        println!("Math: x + y = {}",x+y);
    }
    //println!("Math: x + y = {}",x+y); (it wont print out, not in scope)

    //shadowing
    let a: i32 = 1;
    { 
        let a: i32 = 2;
        println!("{}",a);
    }
    println!("{}",a);
    
    let b: i32 = 1;
    let b: &str = "Hello";
    println!("{}",x);
    
    //remove warnings 
    //#![allow(unused)] -> shebang 
    
    //Suffixes - specify the type of a numeric literal
    let x: i32 = 42;
    let y: u32 = 42;
    println!("{}",x);
    println!("{}",y);
    let z: u32 = 42_u32;
    

}
