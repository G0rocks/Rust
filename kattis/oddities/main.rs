/*
Author:         Huldar
Date:           30.7.2020
Description:
Some numbers are just, well, odd. For example, the number 3 is odd, because it is not a multiple of two.
Numbers that are a multiple of two are not odd, they are even. More precisely, if a number n can be
expressed as n=2⋅k for some integer k, then n is even. For example, 6=2⋅3 is even.
Some people get confused about whether numbers are odd or even. To see a common example, do an internet
search for the query “is zero even or odd?” (Don’t search for this now! You have a problem to solve!)

Write a program to help these confused people

Input
Input begins with an integer 1≤n≤20 on a line by itself, indicating the number of test cases that follow.
Each of the following n lines contain a test case consisting of a single integer −10≤x≤10.

Output
For each x, print either ‘x is odd’ or ‘x is even’ depending on whether x is odd or even.

Sample Input 1	Sample Output 1
3                   10 is even
10                  9 is odd
9                   -5 is odd
-5
*/

//Declare dependencies
use std::io::stdin;

fn main() {
    //take in user input
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    //Declare an array with the same length as the number of expected inputs
    let vector_length:i8 = input_string.trim().parse::<i8>().unwrap();
    let mut input_vector = vec![];      //Create a vector which is an array with a length which is unknown at compile time

    //Change the input string to a number to know how many lines we need to read more
    for i in 0..vector_length {
        input_string.clear();
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        input_vector.push(input_string.trim().parse::<i8>().unwrap());
    }

    //Check if each input is even or odd and output which it is
    for i in 0..vector_length {
        print_even_or_odd(input_vector[i as usize]);
    }
}

fn print_even_or_odd(my_number: i8) {
    if my_number%2 == 0 {
        print!("{} is even\n", my_number);
    }
    else {
        print!("{} is odd\n", my_number);
    }
}
