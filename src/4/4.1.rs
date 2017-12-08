// --- Day 4: High-Entropy Passphrases ---

// A new system policy has been put in place that requires all accounts to use a passphrase instead of simply a password. A passphrase consists of a series of words (lowercase letters) separated by spaces.

// To ensure security, a valid passphrase must contain no duplicate words.

// For example:

// aa bb cc dd ee is valid.
// aa bb cc dd aa is not valid - the word aa appears more than once.
// aa bb cc dd aaa is valid - aa and aaa count as different words.
// The system's full passphrase list is available as your puzzle input. How many passphrases are valid?

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let fname = "src/4/data";

    let input = File::open(fname)
        .expect("Couldn't open file");

    let reader = BufReader::new(&input);
    let mut valids = 0;
    for line in reader.lines().map(|l| l.expect("Couldnt parse a line")) {
        let mut memo = HashMap::new();
        let mut valid = true;
        println!("----------------");
        for word in line.split(" ") {
            let mut chars = word.chars().collect::<Vec<char>>();
            chars.sort();
            let canonical = chars.into_iter().collect::<String>();
            println!("{}", canonical);

            if memo.contains_key(&canonical) {
                println!("matched {}", canonical);
                valid = false;
                break;
            }
            else {
                memo.insert(canonical, 1);
            }
        }
        if valid {
            valids += 1;
        }
    }
    println!("{} valid passwords", valids);
}