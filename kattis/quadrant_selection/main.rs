/*
    Author:                 Huldar
    Date:                   28.07.2020
    Link to problem:        https://open.kattis.com/problems/quadrant
    Problem description:
    A common problem in mathematics is to determine which quadrant a
    given point lies in. There are four quadrants, numbered from 1
    to 4, as shown in the diagram below:
    For example, the point A, which is at coordinates (12,5) lies in
    quadrant 1 since both its x and y values are positive, and point
    B lies in quadrant 2 since its x value is negative and its y value is positive.
    Your job is to take a point and determine the quadrant it is in.
    You can assume that neither of the two coordinates will be 0.
    sample input        sample output
    10                      1
    4
*/

//Declaring dependencies
use std::io;        //In order to be able to take input from the user

fn main() {
    let mut x_string = String::new();
    let mut y_string = String::new();
    let mut x: i16;
    let mut y: i16;

    //Read x coordinate
    io::stdin().read_line(&mut x_string)
    .ok()
    .expect("Failed to read line");

    //Read y coordinate
    io::stdin().read_line(&mut y_string)
    .ok()
    .expect("Failed to read line");

    //Parsing the user input to integer variables. We trim the end to get rid of the newline character
    x = x_string.trim().parse().unwrap();
    y = y_string.trim().parse().unwrap();

    //print!("x_string: {}y_string: {}", x_string, y_string);
    //print!("x: {}, y: {}",x, y);

    if x > 0 {
        if y > 0 {
            print!("1");
        }
        else {
            print!("4");
        }
    }
    else {
        if y > 0 {
            print!("2");
        }
        else {
            print!("3");
        }
    }
}
