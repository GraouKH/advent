use std::{fs, char, collections::HashMap};

fn find_first_number<T: Iterator>(mut chars: T, hash: &HashMap<&str, char>) -> char 
where
    T: Iterator<Item = char>,
{
    let mut substring: String = String::new();
    while let Some(c) = chars.next() {
        if c.is_numeric() {
            return c;
        };
        substring.push(c);
        for (key, value) in hash {
            if substring.ends_with(key) {
                return *value;
            }
        };
    }
    return '0';
}

fn main() {
    let input = fs::read_to_string("../inputs/input1").expect("Read error");

    let mut sum: i32 = 0;
    let hash: HashMap<&str, char> = HashMap::from([("one", '1'),("two", '2'),("three", '3'),("four", '4'),("five", '5'),("six", '6'),("seven", '7'),("eight", '8'),("nine", '9')]);
    let hash_rev: HashMap<&str, char> = HashMap::from([("eno", '1'),("owt", '2'),("eerht", '3'),("ruof", '4'),("evif", '5'),("xis", '6'),("neves", '7'),("thgie", '8'),("enin", '9')]);

    for line in input.lines() {
        let first = find_first_number(line.chars(), &hash);
        let last = find_first_number(line.chars().rev(), &hash_rev);
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }

    println!("{:?}", sum);
}
