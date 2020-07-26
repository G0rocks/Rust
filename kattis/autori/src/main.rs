/*
    Author: Huldar
    date:   26 july 2020
    Description:
        Great scientific discoveries are often named by the last names of scientists that made them.
        For example, the most popular asymmetric cryptography system, RSA was discovered by Rivest,
        Shamir and Adleman. Another notable example is the Knuth-Morris-Pratt algorithm, named by
        Knuth, Morris and Pratt. Scientific papers reference earlier works a lot and it’s not uncommon
        for one document to use two different naming conventions: the short variation (e.g. KMP)
        using only the first letters of authors last names and the long variation (e.g. Knuth-Morris-Pratt)
        using complete last names separated by hyphens.
        We find mixing two conventions in one paper to be aesthetically unpleasing and would like you to
        write a program that will transform long variations into short.
        
        Input
        The first and only line of input will contain at most 100 characters, uppercase and lowercase
        letters of the English alphabet and hyphen (‘-’ ASCII 45). The first character will always be an
        uppercase letter. Hyphens will always be followed by an uppercase letter.
        All other characters will be lowercase letters.
        
        Output
        The first and only line of output should contain the appropriate short variation.
        Sample Input 1	    Sample Output 1
        Knuth-Morris-Pratt  KMP
        Sample Input 2	    Sample Output 2
        Mirko-Slavko        MS
        Sample Input 3	    Sample Output 3
        Pasko-Patak         PP
*/

//Declaring dependencies
use std::io;        //In order to be able to take input from the user

//Declaring constants
const MAX_CHARACTERS: usize = 4*100;      //4 times how many characters you allow because each character is 4 bytes

fn main() {
    //get input a string
    let mut input_string = String::new();

    //print!("input string:\n");  //New line in order to flush the print before asking for user input

    io::stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    //Check and adjust length of string
    let mut input_length: usize = input_string.chars().count();
    if input_length > MAX_CHARACTERS {
        input_length = MAX_CHARACTERS;
    }

    //create output string by taking first letter after each '-' and append it to the string
    let mut output_string = String::new();
    output_string.push(input_string.chars().nth(0).unwrap());   //The first letter will always be first

    for i in 0..(input_length-1) {
        match input_string.chars().nth(i).unwrap() {
            '-' => {
                output_string.push(input_string.chars().nth(i+1).unwrap());    
            },
            _ => {}
        }
    }

    //print output string
    //print!("output string {}", output_string);
    print!("{}",output_string);
}
