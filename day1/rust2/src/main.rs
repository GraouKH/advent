use std::{fs, char};

fn find_first_number<T: Iterator>(mut chars: T) -> char 
where
    T: Iterator<Item = char>,
{
    while let Some(c) = chars.next() {
        if c.is_numeric() {
            return c;
        }
    }
    return '0';
}

fn main() {
    let input = fs::read_to_string("../inputs/exemple2").expect("Read error");

    let mut sum: i32 = 0;
    for line in input.lines() {
        // let mut substring: String = String::new();
        let first = find_first_number(line.chars());
        let last = find_first_number(line.chars().rev());
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }

    println!("{:?}", sum);
}
