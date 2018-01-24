extern crate advent2017 as lib;
use self::lib::particle_parser as parser;
use self::lib::particles;

use std::fs::File;
use std::io::{BufRead, BufReader};
fn parse() -> Vec<particles::Particle3> {
    let mut buf = BufReader::new(File::open("src/20/data").unwrap());
    let mut vec = vec![];
    for line in buf.lines() {
        vec.push(parser::parse_Particle(&line.unwrap()).unwrap());
    }
    vec
}

fn main() {
    for p in parse() {
        println!("{:?}", p);
    }
}
