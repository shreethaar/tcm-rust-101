#![allow(unused)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let a = 5;
    let b = 10;
    let c = true;
    let d = false;

    println!("a > b: {}", a > b); // false
    println!("a >= b: {}", a >= b); // false
    println!("a < b: {}", a < b); // true
    println!("a <= b: {}", a <= b); // true
    println!("a == b: {}", a == b); // false
    println!("a != b: {}", a != b); // true
    println!("True or False: {}", c || d); //true
    println!("True or True: {}", c || c); //true
    println!("False or False: {}", d || d); //false
    println!("True and False: {}", c && d); //false
    println!("True and True: {}", c && c); //true
    println!("False and False: {}", d && d); //false
                        
    //if, else if, else
    println!("How much money do you have?");
    let mut input_money = String::new();
    io::stdin().read_line(&mut input_money);

    let money: i32 = input_money.trim().parse().expect("Entry was not an integer");

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    if (age >= 21) && (money >= 5) {
        println!("We're getting a drink!")
    } else if (age >=21) && (money < 5) {
        println!("Come back with more money!")
    } else if (age < 21) && (money >= 5) {
        println!("Nice try, kid!")
    } else {
        println!("You're too young and too poor.")
    };

    //match - matching arm & all possible values must be covered
    let candidacy_age = 33;
    match candidacy_age {
        1..=24=>println!("Cannot hold office."),
        25..=29=>println!("Can run for the House."),
        //25 | 26 | 27 | 28 | 29 => println!("Can run for the House."),
        30..=34=>println!("Can run for the Senate."),
        35..=i32::MAX=>println!("Can run for President."),
        _=>println!("Are you an infant?")
    };

    let my_age = 33;
    let drinking_age = 21;
    match my_age.cmp(&drinking_age) {
        Ordering::Less=>println!("cannot drink!"),
        Ordering::Equal=>println!("woo,you can drink!"),
        Ordering::Greater=>println!("can drink!"),
    };

    //loops - while, for, infinite loop
    // for loops - start to finish of an iterate

    let mut vegetables = ["Cucumber","Spinach","Cabbage"];
    for x in vegetables.iter(){
        println!("{}",x);
    }

    //loop - infinite loops
    let mut y=0;
    println!("counting");
    loop {
        y+=1;
        println!("{}",y);
        if y==10 {
            println!("We've reached 10");
            //continue; (will continue)
            break;
        }
        if y==20 {
            println!("Reached 20, exit program");
            break;
        }
        

    

}
