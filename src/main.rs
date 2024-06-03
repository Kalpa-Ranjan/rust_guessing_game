/*
*********Guessing Game*********
by Kalpa Behera
*/
// Import library
use std::io;                            // Import IO module from std library.
use rand::prelude::*;                   // Import all functions and methods from random library.
fn main() {
    
    // Take input from user.
    let input = String::new();  // 
    

    // Make a list of fruits
    let guess_list = ["grapes","banana", "oranges"];

    // Generate Random Number
    //----------------------------------------------
    let mut rng = thread_rng();                     // It will help to generate random number
    let index = rng.gen_range(0..guess_list.len());     // It will generate a random number between 0 and length of guess_list(o included but length of guess_list not included)

    // --------------------------------------
    println!("{}",index);

}
