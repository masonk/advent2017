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
#[derive(PartialEq, Eq, Debug)]
struct DancePosition {
    state: u64,
    index: HashMap<u8, usize>,
}

impl DancePosition {
    // x,y refer to partner values
    // a,b refer to indexes
    fn new() -> Self {
        let mut pos = DancePosition {
            state: 0,
            index: HashMap::new(),
        };
        for i in 0..16 {
            pos.set(&i, &(i as u8));
            pos.index.insert(i as u8, i);
        }
        pos
    }
    fn get(&self, idx: &usize) -> u8 {
        let shift = 4 + (*idx * 4) as u32;
        let shift_val = self.state.rotate_left(shift);
        let val: u8 = (shift_val as u8) & 15;
        val
    }
    fn val_to_char(val: &u8) -> char {
        lazy_static! {
            static ref CHARS : Vec<char> = "abcdefghijklmnop".chars().collect::<Vec<char>>();
        }
        CHARS[*val as usize]
    }
    fn char_to_val(c: &char) -> u8 {
        lazy_static! {
            static ref VALS : HashMap<char, u8>  = {
                let mut map = HashMap::new();
                for (v, k) in "abcdefghijklmnop".chars().enumerate() {
                    map.insert(k, v as u8);
                }
                map
            };
        }
        *VALS.get(c).unwrap()
    }
    fn display(&self) -> String {
        let chars = (0..16)
            .map(|i| DancePosition::val_to_char(&self.get(&i)))
            .collect::<String>();
        chars
    }

    fn set(&mut self, idx: &usize, bits: &u8) {
        let shift = 4 + (*idx * 4) as u32;
        let shift_val = self.state.rotate_left(shift);
        let mut mask = shift_val & std::u64::MAX - 15;
        mask += *bits as u64;
        let unshifted = mask.rotate_right(shift);
        self.state = unshifted;
    }
    fn apply_all<'a, M: Iterator<Item = &'a Move>>(&mut self, ms: M) {
        for m in ms {
            self.apply(m)
        }
    }
    fn apply(&mut self, m: &Move) {
        match m {
            &Spin(x) => {
                self.state = self.state.rotate_right((x * 4) as u32);
                for (k, v) in self.index.iter_mut() {
                    *v = (*v + x) % 16;
                }
            }
            &Exchange(a, b) => {
                let x = self.get(&a);
                let y = self.get(&b);

                self.index.insert(x, b);
                self.index.insert(y, a);

                self.set(&b, &x);
                self.set(&a, &y);
            }
            &Partner(p, q) => {
                let x = DancePosition::char_to_val(&p);
                let y = DancePosition::char_to_val(&q);

                let a = *self.index.get(&x).unwrap();
                let b = *self.index.get(&y).unwrap();

                self.index.insert(x, b);
                self.index.insert(y, a);

                self.set(&b, &x);
                self.set(&a, &y);
            }
        }
    }
}

#[cfg(test)]
mod test_dance_positions {
    use DancePosition;
    use Move::*;
    #[test]
    fn test_new() {
        let pos = DancePosition::new();
        let actual = format!("{:064b}", pos.state);
        let expected = (0..16)
            .map(|v| format!("{:04b}", v))
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(actual, expected);
    }
    #[test]
    fn set_n_get() {
        let mut pos = DancePosition::new();
        for i in 0..16 {
            for j in 0..16 {
                pos.set(&i, &j);
                assert_eq!(pos.get(&i), j);
            }
        }
    }
    #[test]
    fn display() {
        let pos = DancePosition::new();
        assert_eq!(pos.display(), "abcdefghijklmnop");
    }
    #[test]
    fn exchange() {
        let mut pos = DancePosition::new();
        pos.apply(&Exchange(3, 7));
        assert_eq!(pos.display(), "abchefgdijklmnop");
    }

    #[test]
    fn spin() {
        let mut pos = DancePosition::new();
        pos.apply(&Spin(5));
        assert_eq!(pos.display(), "lmnopabcdefghijk");
    }
    #[test]
    fn partner() {
        let mut pos = DancePosition::new();
        pos.apply(&Partner('h', 'o'));
        assert_eq!(pos.display(), "abcdefgoijklmnhp");
    }
}
// -- Part Two ---
// Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.

// Keeping the positions they ended up in from their previous dance, the programs perform it again and again: including the first dance, a total of one billion (1000000000) times.

// In the example above, their second dance would begin with the order baedc, and use the same dance moves:

// s1, a spin of size 1: cbaed.
// x3/4, swapping the last two programs: cbade.
// pe/b, swapping programs e and b: ceadb.
// In what order are the programs standing after their billion dances?
fn part_2<'a, M: Iterator<Item = &'a Move>>(moves: M) {
    /// For part 2, we have to apply the input 1e9 times.
    /// It's slow to run the program that many times.
    /// We can pare it down using math.
    /// Note that
    ///     * for any partner operation p_ij = g,
    ///     * for any sequence of spins and index swaps f,
    ///     * for any initial state x,
    /// we have
    ///     g(f(g(x))) = f(x)
    ///
    /// (For all partners x, y, if you swap them, permute the array using spins and index swaps, then swap them again, it's the same result as if they'd never been swapped.)
    ///
    /// Since we're applying each partner exchange an even number of times, they'll all cancel out and can be completely ignored.
    ///
    /// That leaves spins and index swaps, which don't have this nice inverse property and also don't commute.
    /// for example, s1(e0/3(abcd)) = badc but e0/3(s1(abcd)) = dcba
    ///
    /// However, spins and exchanges apply the same effect no matter what partner values are
    /// contained at the index. Therefore, no matter the array that a sequence of spins and and index swaps operates on it, its action
    /// can be expressed using the same linear map.
    ///
    /// To prove this we can directly construct the matrix of the linear map that performs the rearrangement.
    /// Take a smaller case of 5 letters - abcde
    /// Suppose we run our input on it and the result is ebdca
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

    /// Since the application of the inputs is a linear map A, the power of linear algebra becomes available.
    /// Repeated applications of the input are just powers of A. In this case we seek A^1e9.
    ///
    /// At this point it would be possible to throw this to a linear algebra package and have an efficient solution
    /// automatically derived, but the goal of this project is to solve every problem using only very general tools,
    /// such as the standard library.
    ///
    /// The fastest algorithms for this problem rely on the theory of eigenvalues and find exponents of the matrix
    /// by eigendecomposition, which runs in O(r^3), where r is the size of our matrix (16x16). However,
    /// they don't work for all matrices. Our matrix is not provably normal, so it might not be diagonalizable.
    ///
    /// One fairly way that works for all sequences of inputs is to use the fact that A^(x + y) = A^x * A^y
    /// to perform efficient exponentiation by squaring. We start with A and multiply it by A to get A^2.
    /// Squaring A^2 gets us to A^4. We can double our exponent every time and we'll have to do 29 multiplications
    /// before we get to the largest power of 2 that is less than 1e9. We can take advantage of the sparsity of the matrix
    /// by representing it as a hash, and this reduces our time complexity to O(r^2 log n)
    ///
    /// Then we can get exactly 1e9 by multiplying A raised to various powers of 2.
    /// Which powers of 2 do we need? Exactly the powers of 2 that would be "1"s in a binary representation of the number 1e9.
    /// After all, that's what a binary representation of a number _is_: a sum of powers of 2 that equal the number.
    ///
    type SparseMatrix = HashMap<u8, u8>;

    let mut pos = DancePosition::new();
    pos.apply_all(moves.filter(|&m| match m {
        &Move::Partner(_, _) => false, // partner swaps all cancel, see above.
        _ => true,
    }));
    let mut A: SparseMatrix = HashMap::new();
    for i in 0..16 {
        let end = *pos.index.get(&i).unwrap() as u8;
        A.insert(i, end);
    }
    fn multiply(A: &SparseMatrix, B: &SparseMatrix) -> SparseMatrix {
        let mut C = HashMap::new();

        for i in 0..16 {
            let mid = *A.get(&i).unwrap() as u8;
            let end = *B.get(&mid).unwrap() as u8;
            C.insert(end, i);
        }
        C
    }
    fn square(A: &SparseMatrix) -> SparseMatrix {
        return multiply(&A, &A);
    }
    let mut squares: Vec<SparseMatrix> = vec![];
    let highest = 1e9_f64.log2().ceil() as u8;
    squares.push(A);
    for i in 1..highest {
        let B = square(&squares[(i - 1) as usize]);
        squares.push(B);
    }
    let binrep = format!("{:b}", 1_000_000_000);
    let bin = binrep.chars().rev();
    let mut C: SparseMatrix = HashMap::new();
    for i in 0..16 {
        C.insert(i, i);
    }
    for (i, dig) in bin.enumerate() {
        if dig == '1' {
            C = multiply(&C, &squares[i])
        }
    }
    let charmap = "abdefghijklmnop".chars().collect::<Vec<char>>();
    let mut fin = vec!['a'; 16];
    for i in 0..16 {
        let pos = *C.get(&i).unwrap();
        fin[i as usize] = charmap[i as usize];
    }
    println!("{:?}", fin);
}

fn part_1<'a, M: Iterator<Item = &'a Move>>(moves: M) {
    let mut pos = DancePosition::new();
    pos.apply_all(moves);
    println!("{}", pos.display());
}
// You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?
fn main() {
    let f = File::open("src/16/data").unwrap();
    let mut buf = String::new();
    BufReader::new(f).read_line(&mut buf).unwrap();
    let moves = buf.split(",")
        .map(|v| parse_line(v).unwrap())
        .collect::<Vec<Move>>();
    part_1(moves.iter());
    part_2(moves.iter());
}
