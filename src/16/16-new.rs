#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]
use std::io::{BufRead, BufReader, Cursor};
use std::fs::File;
use std::fmt;
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
extern crate regex;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

impl Move {
    fn parse_line(line: &str) -> Option<Self> {
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
    fn parse_moves<R: BufRead>(r: R) -> impl Iterator<Item = Self> {
        let mut buf = String::new();
        BufReader::new(r).read_line(&mut buf).unwrap();
        let moves = buf.split(",")
            .map(|v| Move::parse_line(v).unwrap())
            .collect::<Vec<Move>>();
        moves.into_iter()
    }
}

#[cfg(test)]
mod test {
    use Move;
    use Move::*;
    use std::io::Cursor;
    use std::str;
    #[test]
    fn parse_three_moves() {
        let cursor = Cursor::new(b"s1,x3/4,pe/b");
        let actual = Move::parse_moves(cursor).collect::<Vec<Move>>();
        let expected = vec![Spin(1), Exchange(3, 4), Partner('e', 'b')];
        assert_eq!(actual, expected);
    }
}
// For example, with only five programs standing in a line (abcde), they could do the following dance:
// s1, a spin of size 1: eabcd.
// x3/4, swapping the last two programs: eabdc.
// pe/b, swapping programs e and b: baedc.
// After finishing their dance, the programs end up in order baedc.

struct Dance {
    vals: [char; 16],
    scratch: [char; 16], // a buffer space so that we needn't allocate every time
}

impl Dance {
    fn chars() -> Vec<char> {
        "abcdefghijklmnop".chars().collect::<Vec<char>>()
    }
    fn new() -> Self {
        let mut vals = ['a'; 16];
        for (i, c) in "abcdefghijklmnop".chars().enumerate() {
            vals[i] = c;
        }
        let scratch = ['a'; 16];
        Dance { vals, scratch }
    }
    fn to_string(&self) -> String {
        self.vals.iter().collect::<String>()
    }

    fn t0_index_of(c: &char) -> usize {
        lazy_static!{
            static ref INDEX_OF : HashMap<char, usize> = {
                let mut map = HashMap::new();
                for (i, c) in Dance::chars().iter().enumerate() {
                    map.insert(*c, i);
                }
                map
            };
        }
        *INDEX_OF.get(c).unwrap()
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
        self.scratch[0] = self.vals[*i];
        self.vals[*i] = self.vals[*j];
        self.vals[*j] = self.scratch[0];
    }

    fn exchange_partner(&mut self, a: &char, b: &char) {
        let i = self.vals.iter().position(|v| *v == *a).unwrap();
        let j = self.vals.iter().position(|v| *v == *b).unwrap();
        self.exchange(&i, &j);
    }

    fn apply(&mut self, m: &Move) {
        match m {
            &Spin(x) => self.spin(&x),
            &Exchange(i, j) => self.exchange(&i, &j),
            &Partner(a, b) => self.exchange_partner(&a, &b),
        }
    }

    fn apply_all<'a, M: Iterator<Item = &'a Move>>(&mut self, ms: M) {
        for m in ms {
            self.apply(&m);
        }
    }

    /// Index permutation is a linear map.
    /// Suppose Ax = b
    /// Self is "b" - the vector _after_ permutation
    /// "x" is assumed to be the intial position, "abcdefghijklmnop"
    /// This function solves for A
    fn derive_index_swaps(&self) -> Vec<usize> {
        let mut vec = vec![0; 16];
        for (i, c) in self.vals.iter().enumerate() {
            // suppose the array is "cbadefghijklmnop"
            // Then 0 swapped to 2, and 2 swapped to zero.
            // The array should be [2, 1, 0, 3, 4, 5, ... , 15]
            let start_pos = Dance::t0_index_of(&c);
            // end position is "i"
            vec[start_pos] = i;
        }
        vec
    }

    fn derive_value_swaps(&self) -> Vec<char> {
        let mut vec = vec!['a'; 16];
        let chars = Dance::chars();
        for (i, c) in chars.iter().enumerate() {
            // Where did each char end up?
            let end_idx = self.vals.iter().position(|&v| v == *c).unwrap();
            // what char was there before?
            let swapped_char = chars[end_idx];
            vec[i] = swapped_char;
        }

        // suppose the swaps are
        // a <-> c
        // c <-> e
        // This means a ends where c started,
        // c ends wherever e started,
        // and e ends wherever a started.
        // End position would be
        // [c b e d a ...]
        //
        // Our swaps would be
        // (a b c d e)   <- these chars
        // [e b a d c]  <- go to where these chars were
        // That is, a goes where e was
        // b goes where b was
        // c goes where a was,
        // d goes where d was
        // e goes where c was

        // Suppose swaps are
        // c - e
        // a -c
        // Then end positon would be
        // [c b e d a]
        vec
    }

    fn apply_index_swaps(&mut self, swaps: &Vec<usize>) {
        for (i, j) in swaps.iter().enumerate() {
            self.scratch[*j] = self.vals[i];
        }
        for i in 0..16 {
            self.vals[i] = self.scratch[i];
        }
    }

    fn apply_value_swaps(&mut self, swaps: &Vec<char>) {
        for (i, c) in swaps.iter().enumerate() {
            let j = self.vals.iter().position(|v| v == c).unwrap();
            self.scratch[j] = self.vals[i];
        }
        for i in 0..16 {
            self.vals[i] = self.scratch[i];
        }
    }
}

/// -- Part Two ---
/// Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.

/// Keeping the positions they ended up in from their previous dance, the programs perform it again and again: including the first dance, a total of one billion (1000000000) times.

/// In the example above, their second dance would begin with the order baedc, and use the same dance moves:

/// s1, a spin of size 1: cbaed.
/// x3/4, swapping the last two programs: cbade.
/// pe/b, swapping programs e and b: ceadb.
/// In what order are the programs standing after their billion dances?
///
/// Discussion:
///
/// For part 2, we have to apply the input 1e9 times.
/// It's slow to run the program that many times.
///
/// First decompose the function into two linear maps: the non-partner moves can be reduced to a single
/// permutation matrix that operates on the input.
///
/// Spins and swaps are easy. The operate independently of the contents of the array - they change indexes move indexes.
///
/// For example, we can directly construct matrix of the linear map that performs the rearrangement
/// of spins and swaps for any input.
///
/// Take a smaller case of 5 letters - abcde
/// Suppose we run our spins and swaps on it and the result is ebdca
/// There is a unique, bijective, positive, square matrix A that maps abcde -> ebdca
/// it is:
/*  
        starting index
        0  1  2  3  4  
        ________________            -
    0| 0  0  0  0  1  |  a       |0a + 0b + 0c + 0d + 1e|     e
    1| 0  1  0  0  0  |  b       |0a + 1b + 0c + 0d + 0e|     b
    2| 0  0  0  1  0  |  c    =  |0a + 0b + 0c + 1d + 0e|  =  d
    3| 0  0  1  0  0  |  d       |0a + 0b + 1c + 0d + 0e|     c
    4| 1  0  0  0  0  |  e       |1a + 0b + 0c + 0d + 0e|     a

    */
/// Partner swaps are the opposite. They don't operate on the values of the array.
/// They operate on the _indexes of the values_ of the array.
/// Suppose we run all the partner swaps on "abcde" and get back "ebdca". We now
/// know something about the indexes of the values of the array are changed
///
/*
        starting index
        0  1  2  3  4  
        ________________            
    0| 0  0  0  0  1  |  idx(a)       idx(e)
    1| 0  1  0  0  0  |  idx(b)       idx(b)
    2| 0  0  0  1  0  |  idx(c)    =  idx(d)
    3| 0  0  1  0  0  |  idx(d)       idx(c)
    4| 1  0  0  0  0  |  idx(e)       idx(a)
    */

/// Applying just two permutation functions per iteration gives us a big speedup over applying 1000 ops per iteration
/// but it's not enough. We only need to do this until we find a cycle. This set of permutations that ends in the
/// original state is called the orbit of the permutation.
/// The maximum cycle length of two different permutations over a 16-element permutation group is equivalent
/// to the lcm of two different partitions of 16. Brute force tells us that these partitions are
/// 4,5,7 and 3,13. lcm(4,5,7,3,13) = 3 x 4 x 5 x 7 x 13 = 5460.
/// That's the max cycle length if we get unlucky.
///

#[cfg(test)]
mod dance_test {
    use Dance;
    use Move;
    use get_swaps;
    use Move::*;
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

    #[test]
    fn derive_index_swaps() {
        let mut direct_application = Dance::new();
        let moves = vec![
            Move::Spin(4),
            Move::Exchange(3, 15),
            Move::Exchange(1, 5),
            Move::Exchange(3, 1),
            Move::Spin(3),
        ];

        direct_application.apply_all(moves.iter());

        let index_swaps = direct_application.derive_index_swaps();
        let mut matrix_application = Dance::new();
        matrix_application.apply_index_swaps(&index_swaps);
        assert_eq!(
            matrix_application.to_string(),
            direct_application.to_string()
        );
    }

    #[test]
    fn derive_value_swaps() {
        let mut direct_application = Dance::new();
        let moves = vec![
            Move::Partner('b', 'd'),
            Move::Partner('c', 'p'),
            Move::Partner('b', 'f'),
            Move::Partner('c', 'b'),
        ];

        direct_application.apply_all(moves.iter());
        let value_swaps = direct_application.derive_value_swaps();
        let mut matrix_application = Dance::new();
        matrix_application.apply_value_swaps(&value_swaps);
        assert_eq!(
            matrix_application.to_string(),
            direct_application.to_string()
        );
    }

    #[test]
    fn value_swaps_do_same_as_partner_application() {
        let f = File::open("src/16/data").unwrap();
        let moves = Move::parse_moves(BufReader::new(f))
            .filter(|m| match m {
                &Partner(_, _) => true,
                _ => false,
            })
            .collect::<Vec<Move>>();
        let mut partner_dance = Dance::new();
        partner_dance.apply_all(moves.iter());
        let value_swaps = partner_dance.derive_value_swaps();

        let mut dance = Dance::new();
        dance.apply_value_swaps(&value_swaps);

        let mut direct_dance = Dance::new();
        direct_dance.apply_all(moves.iter());

        assert_eq!(dance.to_string(), direct_dance.to_string());
    }

    #[test]
    fn index_swaps_same_as_spin_application() {
        let f = File::open("src/16/data").unwrap();
        let moves = Move::parse_moves(BufReader::new(f))
            .filter(|m| match m {
                &Partner(_, _) => false,
                _ => true,
            })
            .collect::<Vec<Move>>();
        let mut spin_dance = Dance::new();
        spin_dance.apply_all(moves.iter());
        let index_swaps = spin_dance.derive_index_swaps();

        let mut dance = Dance::new();
        dance.apply_index_swaps(&index_swaps);

        let mut direct_dance = Dance::new();
        direct_dance.apply_all(moves.iter());

        assert_eq!(dance.to_string(), direct_dance.to_string());
    }

    #[test]
    fn direct_application_same_as_matrix() {
        let (index_swaps, value_swaps) = get_swaps();

        let mut dance = Dance::new();
        dance.apply_value_swaps(&value_swaps);
        dance.apply_index_swaps(&index_swaps);

        let mut direct_dance = Dance::new();
        direct_dance.apply_all(moves.iter());

        assert_eq!(dance.to_string(), direct_dance.to_string());
    }
}

fn get_swaps() -> (Vec<usize>, Vec<char>) {
    let f = File::open("src/16/data").unwrap();
    let moves = Move::parse_moves(BufReader::new(f)).collect::<Vec<Move>>();
    let mut spin_dance = Dance::new();
    spin_dance.apply_all(moves.iter().filter(|&m| match m {
        &Partner(_, _) => false,
        _ => true,
    }));

    let mut partner_dance = Dance::new();
    partner_dance.apply_all(moves.iter().filter(|&m| match m {
        &Partner(_, _) => true,
        _ => false,
    }));
    let index_swaps = spin_dance.derive_index_swaps();
    let value_swaps = partner_dance.derive_value_swaps();
    (index_swaps, value_swaps)
}
fn part_one() -> String {
    let (index_swaps, value_swaps) = get_swaps();
    let mut dance = Dance::new();
    dance.apply_value_swaps(&value_swaps);
    dance.apply_index_swaps(&index_swaps);

    dance.to_string()
}

fn part_two() -> String {
    let (index_swaps, value_swaps) = get_swaps();
    let mut dance = Dance::new();
    dance.apply_value_swaps(&value_swaps);
    dance.apply_index_swaps(&index_swaps);
    let mut cycle: Vec<String> = vec![];
    cycle.push(dance.to_string());

    loop {
        dance.apply_value_swaps(&value_swaps);
        dance.apply_index_swaps(&index_swaps);
        let d = dance.to_string();
        if d == cycle[0] {
            for (i, c) in cycle.iter().enumerate() {
                println!("{}. {}", i, c);
            }
            let idx = 1e9 as usize % cycle.len();
            return cycle[idx].clone();
        }
        cycle.push(d);
    }
}
fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
