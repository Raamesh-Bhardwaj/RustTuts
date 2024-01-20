#![allow(unused)]

use std::io;
 
fn main() {
    println!("What is your name?");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Didn't Recieve Name!");
    println!("Hello {}!! {}", name.trim_end(), "Nice to Fucking Meet You!");
}
