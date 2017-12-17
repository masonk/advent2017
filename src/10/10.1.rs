// --- Day 10: Knot Hash ---

// You come across some programs that are trying to implement a software emulation of a hash based on knot-tying. The hash these programs are implementing isn't very strong, but you decide to help them anyway. You make a mental note to remind the Elves later not to invent their own cryptographic functions.

// This hash function simulates tying a knot in a circle of string with 256 marks on it. Based on the input to be hashed, the function repeatedly selects a span of string, brings the ends together, and gives the span a half-twist to reverse the order of the marks within it. After doing this many times, the order of the marks is used to build the resulting hash.

//   4--5   pinch   4  5           4   1
//  /    \  5,0,1  / \/ \  twist  / \ / \
// 3      0  -->  3      0  -->  3   X   0
//  \    /         \ /\ /         \ / \ /
//   2--1           2  1           2   5
// To achieve this, begin with a list of numbers from 0 to 255, a current position which begins at 0 (the first element in the list), a skip size (which starts at 0), and a sequence of lengths (your puzzle input). Then, for each length:

// Reverse the order of that length of elements in the list, starting with the element at the current position.
// Move the current position forward by that length plus the skip size.
// Increase the skip size by one.
// The list is circular; if the current position and the length try to reverse elements beyond the end of the list, the operation reverses using as many extra elements as it needs from the front of the list. If the current position moves past the end of the list, it wraps around to the front. Lengths larger than the size of the list are invalid.

// Here's an example using a smaller list:

// Suppose we instead only had a circular list containing five elements, 0, 1, 2, 3, 4, and were given input lengths of 3, 4, 1, 5.

// The list begins as [0] 1 2 3 4 (where square brackets indicate the current position).
// The first length, 3, selects ([0] 1 2) 3 4 (where parentheses indicate the sublist to be reversed).
// After reversing that section (0 1 2 into 2 1 0), we get ([2] 1 0) 3 4.
// Then, the current position moves forward by the length, 3, plus the skip size, 0: 2 1 0 [3] 4. Finally, the skip size increases to 1.

// The second length, 4, selects a section which wraps: 2 1) 0 ([3] 4.
// The sublist 3 4 2 1 is reversed to form 1 2 4 3: 4 3) 0 ([1] 2.
// The current position moves forward by the length plus the skip size, a total of 5, causing it not to move because it wraps around: 4 3 0 [1] 2. The skip size increases to 2.
// The third length, 1, selects a sublist of a single element, and so reversing it has no effect.
// The current position moves forward by the length (1) plus the skip size (2): 4 [3] 0 1 2. The skip size increases to 3.
// The fourth length, 5, selects every element starting with the second: 4) ([3] 0 1 2. Reversing this sublist (3 0 1 2 4 into 4 2 1 0 3) produces: 3) ([4] 2 1 0.
// Finally, the current position moves forward by 8: 3 4 2 1 [0]. The skip size increases to 4.
// In this example, the first two numbers in the list end up being 3 and 4; to check the process, you can multiply them together to produce 12.
// 0. ([0] 1 2) 3 4

// 1. 2 1) 0 ([3] 4

// 2. 4) ([3] 0 1 2

// 3. 3) ([4] 2 1 0

// However, you should instead use the standard list size of 256 (with values 0 to 255) and the sequence of lengths in your puzzle input. Once this process is complete, what is the result of multiplying the first two numbers in the list?
use std::fmt;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::io::Read;
use std::fmt::Write;

struct Knot {
    list: Vec<u32>,
    cursor: u32,
    skip: u32
}

impl fmt::Debug for Knot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for (i, v) in self.list.iter().enumerate() {
            let s = match i {
                _ if i as u32 == self.cursor => format!("[{}] ", v),
                _ => format!("{} ", v)
            };

            output.push_str(&s);
        }
        write!(f, "{}", &output)
    }
}

impl Knot {
    fn new() -> Knot {
        let mut list = Vec::with_capacity(256);

        for i in 0..256 { list.push(i) }
        Knot {
            list,
            cursor: 0,
            skip: 0
        }
    }

    fn next(&mut self, length: u32) {
        let left = self.cursor;
        let right_unmod = left + length - 1;

        let floor : u32 = (length as f32 / 2.0).floor() as u32;
        let list_ptr = &mut self.list as *mut Vec<u32>;
        for i in 0..floor {
            unsafe {
                let li = self.wrap_offset(left + i) as usize;
                let ri = self.wrap_offset(right_unmod - i) as usize;
                let lv = (*list_ptr)[li];
                (*list_ptr)[li] = (*list_ptr)[ri];
                (*list_ptr)[ri] = lv;
            }
        }
        self.cursor = (self.cursor + length + self.skip) % (self.list.len() as u32);
        self.skip += 1;
    }
    
    fn wrap_offset(&self, o: u32) -> usize {
        (o % (self.list.len() as u32)) as usize
    }
    fn right(&self, length: u32) -> u32 {
        if length == 0 {
            return self.cursor;
        }
        self.wrap_offset(self.cursor + length - 1) as u32
    }

    fn debug(&self, length: u32) {
        let left = self.cursor;
        let right = self.right(length);

        let mut output = String::new();
        for (i, v) in self.list.iter().enumerate().map(|(i, v)| (i as u32, v)) {
            let s = match i {
                _ if i == left && i == right => format!("([{}]) ", v),
                _ if i == left => format!("([{}] ", v),
                _ if i  == right => format!("{}) ", v),
                _ => format!("{} ", v)
            };

            output.push_str(&s);
        }
        println!("{}", &output);
    }
}

fn sample () {
    println!("--------------- SAMPLE ------------");
    let mut small = Knot { list: vec![0, 1, 2, 3, 4], ..Knot::new() };
    for i in [3,4,1,5].iter() {
        small.debug(*i);
        small.next(*i);
        println!("{:?}", small);
    }
    println!("{} * {} = {}", small.list[0], small.list[1], small.list[0] * small.list[1]);

    println!("--------------- END SAMPLE ------------\n");
}

fn part_one() {
    println!("--------------- PART ONE ------------");
    let input = File::open("src/10/data")
        .expect("Couldn't open file");
    let mut reader = BufReader::new(&input);
    let mut knot = Knot::new();
    let mut line = String::new();
    let inputs = reader
        .read_line(&mut line);

    let nums = line.split(",")
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    
    for num in nums.iter() {
        knot.next(*num);
    }
    
    println!("{} * {} = {}", knot.list[0], knot.list[1], knot.list[0] * knot.list[1]);
    println!("--------------- END PART ONE --------\n");
}

fn part_two() {
    let mut knot = Knot::new();
    let mut buf = [0 as u8; 1];
    println!("--------------- PART TWO: DENSE HASH --------");
    for _ in 0..64 {
        let mut f = File::open("src/10/data")
            .expect("Couldn't open file");
        while let Ok(n) = f.read(&mut buf) {
            if n == 0 { break }
            knot.next(buf[0] as u32);
        }
        for num in [17, 31, 73, 47, 23].iter() {
            knot.next(*num);
        }
    }
    // println!("{:?}", knot);
    // println!("--------------- PART TWO: DENSE HASH --------");
    let mut dense = vec![];
    for o in 0..16 {
        let offset = o * 16;
        let mut val : u8 = 0;
        for i in 0..16 {
            val = val ^ (knot.list[offset + i] as u8);
        }
        dense.push(val);
    }
    let mut s = String::new();
    for &byte in dense.iter() {
        write!(&mut s, "{:02X}", byte).unwrap();
    }
    println!("{}", s.to_lowercase());
    println!("--------------- END PART TWO ----------------\n");
}

fn main() {
    sample();
    part_one();
    part_two();
}
