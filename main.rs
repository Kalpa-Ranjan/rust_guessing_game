/*
*********Guessing Game*********
by Kalpa Behera
*/
// Import library
use std::io;                            // Import IO module from std library.
use rand::prelude::*;                   // Import all functions and methods from random library.
fn main() {


    

    // Make a list of fruits
    let guess_list = ["grapes","banana", "oranges"];

    // Generate Random Number
    //----------------------------------------------
    let mut rng = thread_rng();                     // It will help to generate random number
    let index = rng.gen_range(0..guess_list.len());     // It will generate a random number bnetween(0 included but length of guess_list not included)

    let random_fruit = guess_list[index];
    // --------------------------------------

    // Take input from user.
    // ------------------------------------
    let mut input = String::new();  // 
    // -----------------------------
    // Start of Check the input value
    // -------------------------------------- 
    loop{
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{
            
            let fruit_selected = input.trim().to_lowercase();
            println!("fruit_selected {}",fruit_selected);
            if !guess_list.contains(&fruit_selected.as_str()){
                println!("Please enter fruits from the list");
                continue;
            }

            if guess_checker(&fruit_selected,random_fruit){
                println!("You Won $100,000");
                break;
            }
            else{
                println!("Retry");
            }

        }
        Err(error)=>{
            println!("{}",error);
        }
    }

}   
}

fn guess_checker(a:&str, b:&str)->bool{
    return a == b;
}
