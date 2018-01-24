// --- Day 19: A Series of Tubes ---
// Somehow, a network packet got lost and ended up here. It's trying to follow a routing diagram (your puzzle input), but it's confused about where to go.

// Its starting point is just off the top of the diagram. Lines (drawn with |, -, and +) show the path it needs to take, starting by going down onto the only line connected to the top of the diagram. It needs to follow this path until it reaches the end (located somewhere within the diagram) and stop there.

// Sometimes, the lines cross over each other; in these cases, it needs to continue going the same direction, and only turn left or right when there's no other option. In addition, someone has left letters on the line; these also don't change its direction, but it can use them to keep track of where it's been. For example:

//      |
//      |  +--+
//      A  |  C
//  F---|----E|--+
//      |  |  |  D
//      +B-+  +--+

// Given this diagram, the packet needs to take the following path:

// Starting at the only line touching the top of the diagram, it must go down, pass through A, and continue onward to the first +.
// Travel right, up, and right, passing through B in the process.
// Continue down (collecting C), right, and up (collecting D).
// Finally, go all the way left through E and stopping at F.
// Following the path to the end, the letters it sees on its path are ABCDEF.

// The little packet looks up at you, hoping you can help it find the way. What letters will it see (in the order it would see them) if it follows the path? (The routing diagram is very wide; make sure you view it without line wrapping.)

use std::fs::File;
use std::io::{BufRead, BufReader};
fn parse_input(s: &str) -> Vec<Vec<char>> {
    let mut out = vec![];
    let mut r = BufReader::new(File::open(s).unwrap());

    for line in r.lines() {
        let chars = line.unwrap().chars().collect::<Vec<char>>();
        out.push(chars);
    }
    out
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
    Term,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            Term => Term,
        }
    }
}
use self::Dir::*;

struct Tracker {
    i: usize, // row index
    j: usize, // col index
    dir: Dir,
    map: Vec<Vec<char>>,
}

impl Tracker {
    fn new(map: Vec<Vec<char>>) -> Self {
        Tracker {
            i: 0,
            j: 0,
            dir: Down,
            map,
        }
    }
    fn next(&mut self) {
        match self.dir {
            Up => self.i -= 1,
            Down => self.i += 1,
            Left => self.j -= 1,
            Right => self.j += 1,
            Term => {}
        }
    }
    fn look(&self, dir: &Dir) -> Option<char> {
        let i = self.i;
        let j = self.j;

        match *dir {
            Up => {
                if i != 0 {
                    Some(self.map[i - 1][j])
                } else {
                    None
                }
            }
            Down => {
                if i < self.map.len() - 1 {
                    Some(self.map[i + 1][j])
                } else {
                    None
                }
            }
            Right => {
                if j < self.map[0].len() - 1 {
                    Some(self.map[i][j + 1])
                } else {
                    None
                }
            }
            Left => {
                if j != 0 {
                    Some(self.map[i][j - 1])
                } else {
                    None
                }
            }
            Term => Some(self.map[i][j]),
        }
    }
    fn switch_dir(&mut self) {
        for d in [Up, Down, Left, Right, Term].into_iter() {
            if d.opposite() == self.dir {
                continue;
            }
            if let Some(c) = self.look(d) {
                if c != ' ' {
                    self.dir = d.clone();
                    break;
                }
            }
        }
    }
    fn track(&mut self) -> (Vec<char>, usize) {
        self.j = self.map[0].iter().position(|&c| c == '|').unwrap();
        let mut letters = vec![];
        let mut count = 0;
        loop {
            let c = self.map[self.i][self.j];
            count += 1;
            match c {
                '|' | '-' => {}
                '+' => {
                    self.switch_dir();
                }
                _ => {
                    letters.push(c);
                    self.switch_dir();
                }
            }
            match self.dir {
                Term => {
                    return (letters, count);
                }
                _ => self.next(),
            }
        }
    }
}

fn main() {
    let mut tracker = Tracker::new(parse_input("src/19/data"));
    let (letters, count) = tracker.track();
    println!("{}", letters.into_iter().collect::<String>());
    println!("{} steps", count);
}
