// Import io to allow user to write
use std::io;

fn main() {
    /*
    Function that takes in a string with like this:
    2xjzgsjzfhzhm1
    qhklfjd39rpjxhqtftwopfvrrj2eight
    95btwo
    And takes out the numbers, combines them to 2 digit numbers:
    21
    39
    95
    And sums them
    sum = 21 + 39 + 95 = 155
    Returns the sum
    */
    // Init empty sum
    let mut sum:u32 = 0;
    let mut num_1:char = 'A';
    let mut num_2:char = 'A';
    let mut combined_num:String = String::new();

    // Take in string
    let mut input_string = String::new();
    println!("Input trebuchet calibration document:");
    let mut lines = io::stdin().lines();
    //io::stdin().read_to_end(&mut input_string);   // Make sure to read all the lines
    

    // Go through each character in string, find number and add to sum
    while let Some(mut line) = lines.next() {
        let length: i32 = line.unwrap().trim().parse().unwrap();

    //for line in lines {
        for i in 0..length { //(line.unwrap().len()-1) {     //0..input_string.len() {
            // If number, check if num_1 has number, assign number to num_1 if num_1 is less than 0
            input_string = line.unwrap();
            if input_string.chars().nth(i as usize).unwrap().is_digit(10) {
                if num_1 == 'A' {
                    num_1 = input_string.chars().nth(i as usize).unwrap();
                }
                else {
                    if num_2 == 'A' {
                    num_2 = input_string.chars().nth(i as usize).unwrap();
                    // Combine num_1 and num_2 together to become a 2 digit number. Remember to turn to integer
                    // Add combined number to sum
                    combined_num.push(num_1);
                    combined_num.push(num_2);
                    println!("Num 1: {} and num 2: {}", num_1, num_2);
                    sum = sum + combined_num.parse::<u32>().unwrap();
                    combined_num.clear();

                    // Set num_1 and num_2 to -1
                    num_1 = 'A';
                    num_2 = 'A';
                    }
                }
            }
        }
    }
    // Return sum (printout)
    println!("Sum of calibration values:");
    println!("{}", sum);
}
