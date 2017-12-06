use std::io::{BufReader, BufRead};
use std::fs::File;
extern crate itertools;
use itertools::Itertools;
use itertools::MinMaxResult::{NoElements, OneElement, MinMax};

fn main() {
    let fname = "src/2/data";
    
    let input = File::open(fname)
        .expect("Couldn't open file");
    
    let reader = BufReader::new(&input);
    let mut checksum = 0;
    for line in reader.lines().map(|l| l.expect("Couldnt parse a line")) {
        let nums = line.split("\t").map(|digits| digits.parse::<u32>().expect("Couldn't parse a number"));

        if let MinMax(min,max) = nums.minmax() {
            println!("{} - {} = {}", max, min, max - min);
            checksum += max - min;
        }
    }
    println!("Checksum: {}", checksum);
}