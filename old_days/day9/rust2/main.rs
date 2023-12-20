use std::{fs::File, io::{BufReader, BufRead}};

fn get_next_number(mut oasi: Vec<isize>) -> isize {
    let mut all_vec: Vec<Vec<isize>> = Vec::new();
    all_vec.push(oasi.clone());
    while !oasi.iter().all(|&i| i == 0) {
        let mut vec: Vec<isize> = Vec::with_capacity(oasi.len() - 1);
        let mut oasi_iter = oasi.iter();
        let mut previous_num = oasi_iter.next().unwrap();
        while let Some(current_num) = oasi_iter.next() {
            vec.push(current_num - previous_num);
            previous_num = current_num;
        }
        all_vec.push(vec.clone());
        oasi = vec;
    }

    let mut last_num: isize = 0;
    for oasi in (&all_vec[..all_vec.len()-1]).iter().rev() {
        let new_num = oasi.first().unwrap() - last_num;
        last_num = new_num;
    }
    last_num
}
fn main() {
    let file = File::open("exemple").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();
    let mut sum: isize = 0;
    while let Some(line) = input_lines.next() {
        let oasi: Vec<isize> = line.unwrap().split_whitespace().map(|c| c.parse::<isize>().unwrap()).collect();
        sum += get_next_number(oasi);
    }
    println!("{sum}");
}
