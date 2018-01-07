// --- Day 14: Disk Defragmentation ---
// Suddenly, a scheduled job activates the system's disk defragmenter. Were the situation different, you might sit and watch it for a while, but today, you just don't have that kind of time. It's soaking up valuable system resources that are needed elsewhere, and so the only option is to help it finish its task as soon as possible.

// The disk in question consists of a 128x128 grid; each square of the grid is either free or used. On this disk, the state of the grid is tracked by the bits in a sequence of knot hashes.

// A total of 128 knot hashes are calculated, each corresponding to a single row in the grid; each hash contains 128 bits which correspond to individual grid squares. Each bit of a hash indicates whether that square is free (0) or used (1).

// The hash inputs are a key string (your puzzle input), a dash, and a number from 0 to 127 corresponding to the row. For example, if your key string were flqrgnkx, then the first row would be given by the bits of the knot hash of flqrgnkx-0, the second row from the bits of the knot hash of flqrgnkx-1, and so on until the last row, flqrgnkx-127.

// The output of a knot hash is traditionally represented by 32 hexadecimal digits; each of these digits correspond to 4 bits, for a total of 4 * 32 = 128 bits. To convert to bits, turn each hexadecimal digit to its equivalent binary value, high-bit first: 0 becomes 0000, 1 becomes 0001, e becomes 1110, f becomes 1111, and so on; a hash that begins with a0c2017... in hexadecimal would begin with 10100000110000100000000101110000... in binary.

// Continuing this process, the first 8 rows and columns for key flqrgnkx appear as follows, using # to denote used squares, and . to denote free ones:

// ##.#.#..-->
// .#.#.#.#
// ....#.#.
// #.#.##.#
// .##.#...
// ##..#..#
// .#...#..
// ##.#.##.-->
// |      |
// V      V
// In this example, 8108 squares are used across the entire 128x128 grid.

// Given your actual key string, how many squares are used?

// Your puzzle input is jxqlasbh.
extern crate advent2017;
use advent2017::knot::{Knot};

use std::io::Cursor;
/// how many bits in the binary representation of an unsigned integer  
fn bitcount(i: u32) -> u32 {
    let mut sum = 0;
    let mut val = i;
    for _ in 0..32 {
        sum += val & 1;
        val = val >> 1;
    }
    sum
}

/// how many binary bits in a hex number represented as a string
fn bitcount_line(s: &str) -> u32 {    
    let mut knot = Knot::new();

    let mut bitsum = 0;
    let s = knot.hash(Cursor::new(s));
    for j in 0..(32) {
        let slice = &s[j..j+1];
        let num = u32::from_str_radix(slice, 16).unwrap();
        bitsum += bitcount(num);
    }
    bitsum
}

fn count_hash_seed(s: &str) -> u32 {
    let mut bitsum = 0;
    for i in 0..128 {
        let linehash = &format!("{}-{}", s, i);
        bitsum += bitcount_line(linehash);
    }
    bitsum
}

fn sample() {
    let test = "flqrgnkx";
    println!("{}: {}", test, count_hash_seed(&test));
}

fn part_one() {
    let input = "jxqlasbh";
    println!("{}: {}", input, count_hash_seed(&input));
}
fn main() {
    sample();
    part_one();
}

#[cfg(test)]
mod tests {
    use bitcount;
    #[test]
    fn test_bitcount() {
        assert_eq!(bitcount(0), 0);
        assert_eq!(bitcount(1), 1);
        assert_eq!(bitcount(2), 1);
        assert_eq!(bitcount(3), 2);
        assert_eq!(bitcount(4), 1);
        assert_eq!(bitcount(5), 2);
        assert_eq!(bitcount(6), 2);
        assert_eq!(bitcount(7), 3);
        assert_eq!(bitcount(8), 1);
        assert_eq!(bitcount(65535), 16);
        assert_eq!(bitcount(65536), 1);
        assert_eq!(bitcount(4294967295), 32);
    }

}
