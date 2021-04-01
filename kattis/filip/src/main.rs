/*
    Author: Huldar
    date:   1 april 2021
    Description:
        Mirko has a younger brother, Filip, who just started going to school and is having trouble with numbers.
        To help him get the hang of the number scale, his teacher writes two three-digit numbers.
        She asks Filip to compare those numbers, but instead of interpreting them with
        the leftmost most significant digit, he needs to interpret them the other way around,
        with the most significant digit being the rightmost one.
        He then has to tell the teacher the larger of the two numbers.

        Write a program that will check Filip’s answers.

        Input
        The first and only line of input contains two three-digit numbers, A and B.
        A and B will not be equal and will not contain any zeroes.

        Output
        The first and only line of output should contain the larger of the numbers in the input, compared as described in the task.
        The number should be written reversed, to display to Filip how he should read it.

        Sample Input 1	Sample Output 1
        734 893         398
        Sample Input 2	Sample Output 2
        221 231         132
        Sample Input 3	Sample Output 3
        839 237         938
*/

//Declaring dependencies
use std::io;        //In order to be able to take input from the user


fn main() {
    //get input a string
    let mut input_string = String::new();

    let ascii_offset:i32 = 48;

    // Read user input
    io::stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    // Get numbers a and b from input string
    let a: [char; 3] = [input_string.chars().nth(2).unwrap() , input_string.chars().nth(1).unwrap() , input_string.chars().nth(0).unwrap()];
    let b: [char; 3] = [input_string.chars().nth(6).unwrap() , input_string.chars().nth(5).unwrap() , input_string.chars().nth(4).unwrap()];

    // Compare a and b
    let mut bigger_number = 0;
    for i in 0..3 {
        if a[i] as i32 > b[i] as i32 {
            bigger_number = ((a[0] as i32)-ascii_offset)*100 + ((a[1] as i32)-ascii_offset)*10 + ((a[2] as i32)-ascii_offset);
            break;
        }
        if b[i] as i32 > a[i] as i32 {
            bigger_number = ((b[0] as i32)-ascii_offset)*100 + ((b[1] as i32)-ascii_offset)*10 + ((b[2] as i32)-ascii_offset);
            break;
        }
    }

    let output_string = bigger_number.to_string();

    print!("{}", output_string)
}
