use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::str;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut fname = std::env::current_dir()
        .expect("Couldn't get current_dir()");

    fname.push("data");
    fname.push("input");
    let mut input = File::open(fname)
        .expect("Couldn't open file");

    let mut contents = String::new();
    input.read_to_string(&mut contents)
        .expect("Couldn't read file");


    let digits : Vec<u32> = contents
        .chars()
        .map(|c| c.to_digit(10)
            .expect("Couldn't parse a digit"))
        .collect();
    
    let len = digits.len();
    let offset : usize = len / 2; // set to "1" for the first part of the problem
    let mut sum = 0;
    for (i, c) in contents.chars().enumerate() {
        let next_i = (i + offset) % len;
        if (digits[i] == digits[next_i]) {
            // println!("digits[{}] == digits[{}] ({} == {})", i, next_i, digits[i], digits[next_i]);
            sum += digits[i];
        } else {
            // println!("digits[{}] != digits[{}] ({} != {})", i, next_i, digits[i], digits[next_i]);
        }
    }
    println!("{}", sum);
}

