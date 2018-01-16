#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]
use std::io::{BufRead, BufReader, Cursor};
use std::fs::File;
use std::collections::HashMap;
// --- Day 16: Permutation Promenade ---
// You come upon a very unusual sight; a group of programs here appear to be dancing.

// There are sixteen programs in total, named a through p. They start by standing in a line: a stands in position 0, b stands in position 1, and so on until p, which stands in position 15.

// The programs' dance consists of a sequence of dance moves:

// Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise. (For example, s3 on abcde produces cdeab).
// Exchange, written xA/B, makes the programs at positions A and B swap places.
// Partner, written pA/B, makes the programs named A and B swap places.
#[derive(PartialEq, Eq, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}
use Move::*;
use std::fmt;
extern crate regex;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

fn parse_line(line: &str) -> Option<Move> {
    lazy_static! {
        static ref SPIN: Regex = Regex::new(r"s(\d+)").unwrap();
        static ref EXCHANGE: Regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
        static ref PARTNER: Regex = Regex::new(r"p(\w)/(\w)").unwrap();
    }
    if let Some(cap) = SPIN.captures(line) {
        return Some(Spin(
            str::parse::<usize>(cap.get(1).unwrap().as_str()).unwrap(),
        ));
    }
    if let Some(cap) = EXCHANGE.captures(line) {
        let a = str::parse::<usize>(cap.get(1).unwrap().as_str()).unwrap();
        let b = str::parse::<usize>(cap.get(2).unwrap().as_str()).unwrap();
        return Some(Exchange(a, b));
    }
    if let Some(cap) = PARTNER.captures(line) {
        let a = cap.get(1).unwrap().as_str();
        let b = cap.get(2).unwrap().as_str();
        return Some(Partner(
            a.chars().nth(0).unwrap(),
            b.chars().nth(0).unwrap(),
        ));
    }
    None
}
fn parse_moves<R: BufRead>(r: R) -> impl Iterator<Item = Move> {
    r.lines()
        .map(|v| v.unwrap())
        .map(|v| parse_line(&v))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
}
#[cfg(test)]
mod test {
    use Move;
    use Move::*;
    use std::io::Cursor;
    use parse_moves;
    use std::str;
    #[test]
    fn parse_three_moves() {
        let cursor = Cursor::new(b"s1\nx3/4\r\npe/b");
        let actual = parse_moves(cursor).collect::<Vec<Move>>();
        let expected = vec![Spin(1), Exchange(3, 4), Partner('e', 'b')];
        assert_eq!(actual, expected);
    }
}
// For example, with only five programs standing in a line (abcde), they could do the following dance:
// s1, a spin of size 1: eabcd.
// x3/4, swapping the last two programs: eabdc.
// pe/b, swapping programs e and b: baedc.
// After finishing their dance, the programs end up in order baedc.

// ----
// there are 2^4 programs, so the whole state can be represented in 2^4 * 4 = 64 bits
// This is convenient for spinning - there is an LLVM instrinsic for that - but less convenient
// for swapping and exchanging.
// We'll impl swap(i, j) which exchanges 4 bits at i with 4 bits at j. This provides O(1) index-based swaps.
// To have O(1) partner swaps we'll maintain an index of where each partner is, to allow us to retrieve indexes in O(1).
// Then all operations are O(1).
//

struct Dance {
    vals: Vec<char>,
    scratch: Vec<char>,
}

impl Dance {
    fn new() -> Self {
        let vals = "abcdefghijklmnop".chars().collect::<Vec<char>>();
        let scratch = vec!['a'; 16];
        Dance { vals, scratch }
    }
    fn to_string(&self) -> String {
        self.vals.iter().collect::<String>()
    }

    fn spin(&mut self, x: &usize) {
        let mid = self.vals.len() - *x;

        for i in 0..16 {
            let idx = (i + mid) % 16;
            self.scratch[i] = self.vals[idx];
        }
        for i in 0..16 {
            self.vals[i] = self.scratch[i];
        }
    }

    fn exchange(&mut self, i: &usize, j: &usize) {
        let buf = self.vals[*i];
        self.vals[*i] = self.vals[*j];
        self.vals[*j] = buf;
    }

    fn exchange_partner(&mut self, a: &char, b: &char) {
        let i = self.vals.iter().position(|v| *v == *a).unwrap();
        let j = self.vals.iter().position(|v| *v == *b).unwrap();
        self.exchange(&i, &j);
    }
}

#[cfg(test)]
mod dance_test {
    use Dance;
    #[test]
    fn spin() {
        let mut dance = Dance::new();
        dance.spin(&0);
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.spin(&3);
        assert_eq!(&dance.to_string(), "nopabcdefghijklm");
        dance.spin(&0);
        assert_eq!(&dance.to_string(), "nopabcdefghijklm");
        dance.spin(&5);
        assert_eq!(&dance.to_string(), "ijklmnopabcdefgh");
    }

    #[test]
    fn exchange() {
        let mut dance = Dance::new();
        dance.exchange(&0, &0);
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.exchange(&15, &15);
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.exchange(&15, &0);
        assert_eq!(&dance.to_string(), "pbcdefghijklmnoa");
        dance.exchange(&15, &0);
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.exchange(&0, &15);
        assert_eq!(&dance.to_string(), "pbcdefghijklmnoa");
        dance.exchange(&0, &15);
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
    }

    #[test]
    fn exchange_partner() {
        let mut dance = Dance::new();
        dance.exchange_partner(&'a', &'a');
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.exchange_partner(&'p', &'p');
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.exchange_partner(&'p', &'a');
        assert_eq!(&dance.to_string(), "pbcdefghijklmnoa");
        dance.exchange_partner(&'p', &'a');
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
        dance.exchange_partner(&'a', &'p');
        assert_eq!(&dance.to_string(), "pbcdefghijklmnoa");
        dance.exchange_partner(&'a', &'p');
        assert_eq!(&dance.to_string(), "abcdefghijklmnop");
    }

}
