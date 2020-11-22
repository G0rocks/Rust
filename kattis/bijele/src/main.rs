/*
    Author: Huldar
    date:   29 july 2020
    Description:
    Mirko has found an old chessboard and a set of pieces in his attic. Unfortunately, the set contains only white pieces,
    and apparently an incorrect number of them. A set of pieces should contain:
    One king
    One queen
    Two rooks
    Two bishops
    Two knights
    Eight pawns
    Mirko would like to know how many pieces of each type he should add or remove to make a valid set.

    Input
    The input consists of 6 integers on a single line, each between 0 and 10 (inclusive). The numbers are, in
    order, the numbers of kings, queens, rooks, bishops, knights and pawns in the set Mirko found.

    Output
    Output should consist of 6 integers on a single line; the number of pieces of each type Mirko should add or remove.
    If a number is positive, Mirko needs to add that many pieces. If a number is negative, Mirko needs to remove pieces.

    Sample Input 1  	Sample Output 1
    0 1 2 2 2 7            1 0 0 0 0 1
    Sample Input 2	Sample Output 2
    2 1 2 1 2 1             -1 0 0 1 0 7
*/

//Declaring dependencies
use std::io;        //In order to be able to take input from the user

//Declare constants
const NUM_KINGS:i8 = 1;
const NUM_QUEENS:i8 = 1;
const NUM_ROOKS:i8 = 2;
const NUM_BISHOPS:i8 = 2;
const NUM_KNIGHTS:i8 = 2;
const NUM_PAWNS:i8 = 8;

fn main() {
    //read number of pieces from user input
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)
        .ok()        .expect("Failed to read line");

    //read number of pieces Mirko has found from input_string
    let mut input_string_split = input_string.split_whitespace();

    let mirko_kings:i8 = input_string_split.next().unwrap().parse().unwrap();
    let mirko_queens:i8 = input_string_split.next().unwrap().parse().unwrap();
    let mirko_rooks:i8 = input_string_split.next().unwrap().parse().unwrap();
    let mirko_bishops:i8 = input_string_split.next().unwrap().parse().unwrap();
    let mirko_knights:i8 = input_string_split.next().unwrap().parse().unwrap();
    let mirko_pawns:i8 = input_string_split.next().unwrap().parse().unwrap();

    //Compare number of pieces Mirko has to number of pieces he is supposed to have. Store result
    let add_kings:i8 = NUM_KINGS - mirko_kings;
    let add_queens:i8 = NUM_QUEENS - mirko_queens;
    let add_rooks:i8 = NUM_ROOKS - mirko_rooks;
    let add_bishops:i8 = NUM_BISHOPS - mirko_bishops;
    let add_knights:i8 = NUM_KNIGHTS - mirko_knights;
    let add_pawns:i8 = NUM_PAWNS - mirko_pawns;

    //print results
    print!("{} {} {} {} {} {}",add_kings, add_queens, add_rooks, add_bishops, add_knights, add_pawns);
}