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
#[derive(PartialEq, Eq)]
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
    fn chars() -> &'static Vec<char> {
        lazy_static! {
            static ref CHARS : Vec<char> = "abcdefghijklmnop".chars().collect::<Vec<char>>();
        }
        &CHARS
    }
    fn get(&self, idx: &usize) -> u8 {
        let shift = 4 + (*idx * 4) as u32;
        let shift_val = self.state.rotate_left(shift);
        let val: u8 = (shift_val as u8) & 15;
        val
    }
    fn val_to_char(val: &u8) -> char {
        DancePosition::chars()[*val as usize]
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
    fn cs(&self) -> Vec<char> {
        (0..16)
            .map(|i| DancePosition::val_to_char(&self.get(&i)))
            .collect()
    }

    fn display(&self) -> String {
        self.cs().iter().collect()
    }

    /// the value at "idx" to "bits"
    fn set(&mut self, idx: &usize, bits: &u8) {
        let shift = 4 + (*idx * 4) as u32;
        let shift_val = self.state.rotate_left(shift);
        let mut mask = shift_val & std::u64::MAX - 15;
        mask += *bits as u64;
        let unshifted = mask.rotate_right(shift);
        self.state = unshifted;
    }

    fn from_chars(&mut self, chars: &Vec<char>) {
        for (i, c) in chars.iter().enumerate() {
            let v = DancePosition::char_to_val(c);
            self.set(&i, &v);
            self.index.insert(i as u8, v as usize);
        }
    }

    /// a vec containing [idx(a), idx(b), ... , idx(p) ]
    fn indexes(&self) -> Vec<u8> {
        (*DancePosition::chars())
            .iter()
            .map(|c| DancePosition::char_to_val(c))
            .map(|v| *self.index.get(&v).unwrap() as u8)
            .collect::<Vec<u8>>()
    }

    /// e.g. if indexes is [1, 2, 3, 4, 0]
    /// then the value at 0 is the previous value at 1
    /// the value at 2 is the previous value at 1, etc
    /// the value at 4 is the previous value of 0
    fn reorder_by_indexes(&mut self, indexes: &Vec<u8>) {
        let mut rearrangements = vec![];
        for i in indexes {
            rearrangements.push(self.get(&(indexes[*i as usize] as usize)));
        }
        for (i, v) in rearrangements.iter().enumerate() {
            self.set(&i, &v);
        }
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

    fn apply_matrix(&mut self, m: &SwapMatrix16) {
        let old = &self.cs();
        let new_chars = m.apply_to(&old);
        self.from_chars(&new_chars);
    }

    fn apply_index_permutations(&mut self, m: &SwapMatrix16) {
        let old = self.indexes();
        let new_indices = m.apply_to(&old);
        self.reorder_by_indexes(&new_indices);
    }
}
impl fmt::Debug for DancePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut indexes = self.index.iter().collect::<Vec<(&u8, &usize)>>();
        indexes.sort_by_key(|&(&k, &v)| k);
        let disp = indexes.iter().map(|&(_, &v)| v as u8).collect::<Vec<u8>>();
        write!(
            f,
            "DancePosition {{ \"{}\" (index: {:?}) }}",
            self.display(),
            disp
        )
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

type SparseMatrix = HashMap<u8, u8>;
#[derive(Clone)]
struct SwapMatrix16 {
    // a 16x16 matrix that has a single non-zero entry in each row and column
    entries: SparseMatrix,
}
impl fmt::Debug for SwapMatrix16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut indexes = self.entries.iter().collect::<Vec<(&u8, &u8)>>();
        indexes.sort_by_key(|&(&k, &v)| k);
        let disp = indexes.iter().map(|&(_, &v)| v).collect::<Vec<u8>>();
        write!(f, "Matrix {{ \"{:?}\" }}", disp)
    }
}

impl SwapMatrix16 {
    fn identity() -> Self {
        let mut m: SparseMatrix = HashMap::new();
        for i in 0..16 {
            m.insert(i, i);
        }
        SwapMatrix16 { entries: m }
    }
    fn build_from<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> Self {
        // find the permutation matrix that maps a to b
        let mut entries: SparseMatrix = HashMap::new();
        for (i, start) in a.iter().enumerate() {
            let end = b.iter().position(|v| start == v).unwrap();
            entries.insert(i as u8, end as u8);
        }
        SwapMatrix16 { entries }
    }

    fn set(&mut self, start_idx: u8, end_idx: u8) {
        // tell the matrix to map start_idx to end_idx
        self.entries.insert(start_idx, end_idx);
    }

    /// permute the values of a vector
    fn apply_to<T: Clone>(&self, vec: &Vec<T>) -> Vec<T> {
        // take a vec and build a new vec with the action of the matrix applied
        let mut fin: Vec<T> = Vec::with_capacity(16);
        for _ in 0..16 {
            fin.push(vec[0].clone());
        }
        for i in 0..16 {
            let pos = *self.entries.get(&i).unwrap();
            let v = &vec[i as usize];
            fin[pos as usize] = v.clone();
        }
        fin
    }
}

/// Get a matrix that performs value permutations equivalent to all spin and index swaps of a move set
fn get_non_partner_map<'a, I: Iterator<Item = &'a Move>>(moves: I) -> SwapMatrix16 {
    let mut pos = DancePosition::new();
    let m = moves.filter(|&m| match m {
        &Move::Partner(_, _) => false, // partner swaps all cancel,
        _ => true,
    });
    pos.apply_all(m);
    SwapMatrix16::build_from(DancePosition::chars(), &pos.cs())
}

/// Get a matrix that performs index permutations equivalent to the partner swaps in moves
fn get_partner_map<'a, I: Iterator<Item = &'a Move>>(moves: I) -> SwapMatrix16 {
    let mut pos = DancePosition::new();
    pos.apply_all(moves.filter(|&m| match m {
        &Move::Partner(_, _) => true, // partner swaps all cancel,
        _ => false,
    }));
    SwapMatrix16::build_from(&(0..16u8).into_iter().collect::<Vec<u8>>(), &pos.indexes())
}
extern crate rand;
use rand::Rng;
#[cfg(test)]
mod test_swap_matrix {
    use SwapMatrix16;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use DancePosition;
    use parse_line;
    use Move;
    use rand::Rng;
    use rand;
    use get_non_partner_map;
    use get_partner_map;

    #[test]
    fn identity() {
        let id = SwapMatrix16::identity();
        let rando = "hdncmqialmxiwldj".chars().collect::<Vec<char>>();
        let actual = id.apply_to(&rando);
        assert_eq!(actual, rando);
    }

    #[test]
    fn index_permutations() {
        let start = DancePosition::new();
        let mut end = DancePosition::new();
        end.from_chars(&"abcdmfhijklenop".chars().collect());
        let matrix = SwapMatrix16::build_from(&start.indexes(), &end.indexes());
        println!("{:?} \n{:?} \n{:?}", start, end, matrix);
        for i in 0..16 {
            let to = matrix.entries.get(&i).unwrap();
            if i == 4 {
                assert_eq!(*to, 12);
            } else if i == 12 {
                assert_eq!(*to, 4);
            } else {
                assert_eq!(*to, i);
            }
        }
    }

    #[test]
    fn build_matrix_from_abcd() {
        let mut rng = rand::thread_rng();
        for _ in 0..128 {
            let a = DancePosition::chars();
            let mut b = a.clone();
            rng.shuffle(&mut b);
            let m = SwapMatrix16::build_from(a, &b);
            assert_eq!(m.apply_to(a), b);
        }
    }
    #[test]
    fn build_matrix_from_random_points() {
        let mut rng = rand::thread_rng();
        for _ in 0..128 {
            let a = DancePosition::chars();
            let mut b = a.clone();
            let mut c = a.clone();
            rng.shuffle(&mut b);
            rng.shuffle(&mut c);
            let m = SwapMatrix16::build_from(&b, &c);
            assert_eq!(m.apply_to(&b), c);
        }
    }

    #[test]
    fn spin_matrix_same_as_spin_moves() {
        let f = File::open("src/16/data").unwrap();

        let mut buf = String::new();
        BufReader::new(f).read_line(&mut buf).unwrap();
        let moves = buf.split(",")
            .map(|v| parse_line(v).unwrap())
            .collect::<Vec<Move>>();

        let spin_matrix = get_non_partner_map(moves.iter().clone());

        let mut raw_pos = DancePosition::new();
        let mut matrix_pos = DancePosition::new();

        for _ in 0..4 {
            raw_pos.apply_all(moves.iter().clone().filter(|&m| match m {
                &Move::Partner(_, _) => false,
                _ => true,
            }));
            matrix_pos.apply_matrix(&spin_matrix);

            assert_eq!(raw_pos.display(), matrix_pos.display());
        }
    }

    #[test]
    fn partner_matrix_same_as_partner_moves() {
        let f = File::open("src/16/data").unwrap();

        let mut buf = String::new();
        BufReader::new(f).read_line(&mut buf).unwrap();
        let moves = buf.split(",")
            .map(|v| parse_line(v).unwrap())
            .collect::<Vec<Move>>();

        let mut raw_pos = DancePosition::new();
        let mut matrix_pos = DancePosition::new();
        let partner_matrix = get_partner_map(moves.iter().clone());

        for _ in 0..1 {
            raw_pos.apply_all(moves.iter().clone().filter(|&m| match m {
                &Move::Partner(_, _) => true,
                _ => false,
            }));
            matrix_pos.apply_index_permutations(&partner_matrix);

            assert_eq!(raw_pos.display(), matrix_pos.display());
        }
    }

    #[test]
    fn matrix_applications_same_as_move_applications() {
        let f = File::open("src/16/data").unwrap();

        let mut buf = String::new();
        BufReader::new(f).read_line(&mut buf).unwrap();
        let moves = buf.split(",")
            .map(|v| parse_line(v).unwrap())
            .collect::<Vec<Move>>();

        let spin_matrix = get_non_partner_map(moves.iter().clone());
        let partner_matrix = get_partner_map(moves.iter().clone());

        let mut raw_pos = DancePosition::new();
        let mut matrix_pos = DancePosition::new();

        for _ in 0..4 {
            raw_pos.apply_all(moves.iter());
            matrix_pos.apply_matrix(&spin_matrix);
            matrix_pos.apply_index_permutations(&partner_matrix);

            assert_eq!(raw_pos.display(), matrix_pos.display());
        }
    }
}

fn part_2<'a, M: Iterator<Item = &'a Move>>(moves: M) {
    let mut pos = DancePosition::new();
    pos.apply_all(moves.filter(|&m| match m {
        &Move::Partner(_, _) => false, // partner swaps all cancel, see discussion.
        _ => true,
    }));
    let mut application_matrix: SwapMatrix16 = SwapMatrix16::identity();
    for i in 0..16 {
        let end = *pos.index.get(&i).unwrap() as u8;
        application_matrix.set(i, end);
    }
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
