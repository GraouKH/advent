use std::{fs, char};

fn main() {
    let input = fs::read_to_string("../1/input").expect("Read error");

    // let mut sum: i32 = 0; 
    // for line in input.lines() {
    //     let mut first = '0';
    //     let mut last = '0';
    //     let mut chars = line.chars();
    //     while let Some(c) = chars.next() {
    //         if c.is_numeric() {
    //             first = c;
    //             break;
    //         }
    //     }
    //     let mut chars_rev = line.chars().rev();
    //     while let Some(c) = chars_rev.next() {
    //         if c.is_numeric() {
    //             last = c;
    //             break;
    //         }
    //     }
    // sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    // }


    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<char> = line.chars().filter(|&c| c.is_numeric()).collect();
        sum += format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse::<i32>().unwrap();
    }
    println!("{:?}", sum);
}
