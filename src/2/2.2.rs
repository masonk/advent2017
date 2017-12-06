use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let fname = "src/2/data";
    
    let input = File::open(fname)
        .expect("Couldn't open file");
    
    let reader = BufReader::new(&input);
    let mut checksum = 0;
    for line in reader.lines().map(|l| l.expect("Couldnt parse a line")) {
        let mut nums = line
            .split("\t")
            .map(|digits| digits.parse::<u32>().expect("Couldn't parse a number"))
            .collect::<Vec<u32>>();

        nums.sort();

        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[j] % nums[i] == 0 {
                    checksum += nums[j] / nums[i];
                    println!("{} / {} = {}", nums[j], nums[i], nums[j] / nums[i]);
                    break;
                }
            }
        }
    }
    println!("{}", checksum);
}