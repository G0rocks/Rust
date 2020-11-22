/*
Author:         Huldar
Date:           28.07.2020
Description:
get 3 points as an input. Output fourth point so that the rectangle between the points is parallel to both x and y axis
sample input    sample output
5 5                 7 7
7 5
5 7
*/

//Declaring dependencies
use std::io;        //In order to be able to take input from the user

fn main() {
    let mut p1_string = String::new();
    let mut p2_string = String::new();
    let mut p3_string = String::new();

    //Read input for points
    io::stdin().read_line(&mut p1_string)
        .ok()
        .expect("Failed to read point 1");
    io::stdin().read_line(&mut p2_string)
        .ok()
        .expect("Failed to read point 2");
    io::stdin().read_line(&mut p3_string)
        .ok()
        .expect("Failed to read point 3");

    //Split string by space to get x and y coordinates of each point
        //Point 1
    let mut p1_string_split = p1_string.split_whitespace();
    let p1x:i16 = p1_string_split.next().unwrap().parse().unwrap();
    let p1y:i16 = p1_string_split.next().unwrap().parse().unwrap();
        //Point 2
    let mut p2_string_split = p2_string.split_whitespace();
    let p2x:i16 = p2_string_split.next().unwrap().parse().unwrap();
    let p2y:i16 = p2_string_split.next().unwrap().parse().unwrap();
        //Point 3
    let mut p3_string_split = p3_string.split_whitespace();
    let p3x:i16 = p3_string_split.next().unwrap().parse().unwrap();
    let p3y:i16 = p3_string_split.next().unwrap().parse().unwrap();

    //Create point 4 coordinates
    let p4x:i16;
    let p4y:i16;

    //check which x coordinate only appears once
    //Input that coordinate to p4_x
    if p1x == p2x {
        p4x = p3x;
    }
    else if p1x == p3x {
        p4x = p2x;
    }
    else {
        p4x = p1x;
    }

    //check which y coordinate only appears once
    //Input that coordinate to p4_y
    if p1y == p2y {
        p4y = p3y;
    }
    else if p1y == p3y {
        p4y = p2y;
    }
    else {
        p4y = p1y;
    }
    /*
    print!("p1x: {} p1y: {}\n", p1x, p1y);
    print!("p2x: {} p2y: {}\n", p2x, p2y);
    print!("p3x: {} p3y: {}\n", p3x, p3y);
    */
    print!("{} {}",p4x,p4y);
}