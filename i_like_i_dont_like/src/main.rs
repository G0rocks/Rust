/*
********************************************************************************************************************
Author          :   Huldar
Date            :   2020-05-23
Program         :   It's a game that asks you what you like & don't like. The rule is that you only like things
                        which are called with a word which contains 2 of the same letter in a row at some point
********************************************************************************************************************
*/

use std::io;

//Function that checks if any of 
fn likes(input: &String) -> bool {
    for i in 0..input.len()-1 {
        //If correct, confirm
        if input.chars().nth(i) == input.chars().nth(i+1) {
            return  true;
        }
    }
    return false;
}

fn main() {
    //Variable declaration
    let mut string_like = String::new();
    let mut string_dont_like = String::new();
    let mut temp_input = String::new();
    let mut bool_continue:char = 'y';

    //Introduction to game
    println!("I like wood but I don't like timber");
    println!("There is a rule to this game. Type in words that you like & don't like and I'll tell you what the reality is");


    //While loop
    while bool_continue.to_ascii_lowercase() == 'y' {

        //Input 1
        println!("I like :");
        io::stdin().read_line(&mut string_like);
        
        //Input 2
        println!("I don't like :");
        io::stdin().read_line(&mut string_dont_like);

        //Check string_like
        if likes(&string_like) {
            print!("You were right, you do like {}", string_like);
        }
        else {
            print!("Actually you don't really care for {}", string_like);
        }
        //Check string_dont_like
        if likes(&string_dont_like) {
            print!("But you really like {}  !", string_dont_like);
        }
        else{
            print!("But nobody likes {}", string_dont_like);
        }

        //Empty strings for next round
        string_dont_like.clear();
        string_like.clear();

        //Continue?
        println!("\nPress \"Enter\" to continue or \"Ctrl+C\" to quit.");
        io::stdin().read_line(&mut temp_input);
    }
}
