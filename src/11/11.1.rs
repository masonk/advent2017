// --- Day 11: Hex Ed ---

// Crossing the bridge, you've barely reached the other side of the stream when a program comes up to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite grid!"

// Fortunately for her, you have plenty of experience with infinite grids.

// Unfortunately for you, it's a hex grid.

// The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the north, northeast, southeast, south, southwest, and northwest:

//   \ n  /
// nw +--+ ne
//   /    \
// -+      +-
//   \    /
// sw +--+ se
//   / s  \
// You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A "step" means to move from the hex you are in to any adjacent hex.)

// For example:

// ne,ne,ne is 3 steps away.
// ne,ne,sw,sw is 0 steps away (back where you started).
// ne,ne,s,s is 2 steps away (se,se).
// se,sw,se,sw,sw is 3 steps away (s,s,sw).

// --- Part Two ---

// How many steps away is the furthest he ever got from his starting position?

// --- solution ---
// Grid position can be represented as a 2d vector with vector addition and scalar 
// multiplication defined, forming a space. Defining distance as the minimum
// number of moves to go from one vector to another adds the necessary metric.
// 
// n moves +1 up and + 0 right.  (1, 0)
// ne moves +0.5 up and +0.5 right (.5, .5)
// nw moves +0.5 up and -.5 right (.5, -.5)
//
// The other three directions invert the values of these three
// Note that it's faster to move up and down than directly l and r.
// You can only move l/r at .5 distance per move, but while doing that you can get a free .5 u/d
// The shortest path is to move  along the diagonal that most closely aims for the target,
// then, on achieving the necessary lateral ddistance, move u/d to reach the target.
// If you need more lateral moves than u/d moves move out on a diagonal until the up 
// direction is reached, then snake back and forth to go "straight out" at the cost of
// 2 moves per one unit of lateral distance.
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
struct HexVector {
    up: f32,
    right: f32
}

impl HexVector {
    fn new() -> HexVector {
        HexVector { up: 0.0, right: 0.0 }
    }

    fn add(&mut self, other: &HexVector) {
        self.up += other.up;
        self.right += other.right;
    }

    fn scalar_multiply(&mut self, scalar: f32) {
        self.up *= scalar;
        self.right *= scalar;
    }

    fn dist(&self, other: &HexVector) -> f32 {
        let lateral = (other.right - self.right).abs();
        let vertical = (other.up - self.right).abs();
        (lateral * 2.0 + (vertical - lateral).max(0.0)).ceil()
    }
}

fn process_move_list<'a, T>(cursor: &mut HexVector, moves: T, start: &HexVector) -> (f32, HexVector)
    where T: IntoIterator<Item = Result<&'a HexVector, String>> {
    let mut max : f32 = 0.0;
    let mut max_pos : HexVector = start.clone();
    for mvr in moves.into_iter() {
        match mvr {
            Ok(mv) => { 
                cursor.add(&mv); 
                let dist = start.dist(cursor);
                if dist > max {
                    max = dist;
                    max_pos = cursor.clone();
                }
            },
            Err(msg) => { println!("{}", msg); }
        };

    }
    (max, max_pos)
}

lazy_static! {
    static ref MOVES: HashMap<&'static str, HexVector> = {
        let mut m = HashMap::new();
        let n = HexVector { up: 1.0, right: 0.0 };
        let mut s = n.clone();
        s.scalar_multiply(-1.0);
        let ne = HexVector { up: 0.5, right: 0.5 };
        let mut sw = ne.clone();
        sw.scalar_multiply(-1.0);
        let nw = HexVector { up: 0.5, right: -0.5 };
        let mut se = nw.clone();
        se.scalar_multiply(-1.0);

        m.insert("n", n);
        m.insert("s", s);
        m.insert("ne", ne);
        m.insert("se", se);
        m.insert("nw", nw);
        m.insert("sw", sw);
        m
    };
}

struct HexVecIterator<'a> {
    stream: &'a mut Read,
}

impl<'a> Iterator for HexVecIterator<'a> {
    type Item = Result<&'a HexVector, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bytes = [0; 1];
        let mut buf = vec![];
        while let Ok(_) = self.stream.read_exact(&mut bytes[..]) {
            if bytes[0] as char == ',' {
                let result : Option<Result<&'a HexVector, String>>;
                {
                    let token = std::str::from_utf8(&buf).unwrap();
                    result = Some(MOVES.get(token).ok_or(format!("couldn't parse {}", token)));
                }
                buf.clear();
                return result;
            }
            buf.push(bytes[0]);
        }
        if buf.len() > 0 {
            let token = std::str::from_utf8(&buf).unwrap();
            return Some(MOVES.get(token).ok_or(format!("couldn't parse {}", token)));
        }
        return None;
    }
}

fn main() {

    let mut f = File::open("src/11/data")
        .expect("Couldn't open file");

    let start = HexVector::new();
    let mut end = start.clone();
    let (max, max_pos) = process_move_list(&mut end, HexVecIterator { stream: &mut f }, &start);
    println!("11-1: Child ended up at {:?} which is {} moves from (0,0).",  end, start.dist(&end));
    println!("11-2: Furthest position occurred at {:?}, which is {} away from (0,0)", max_pos,  max);
}
