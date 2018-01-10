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
#![feature(conservative_impl_trait)]
#![feature(entry_and_modify)]
// #![feature(nll)]
extern crate advent2017;
use advent2017::knot::{Knot};
use std::io::Cursor;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
/// Given any Binary, return an iterator that iterates through the binary
/// representation of the type (msb first), and returns true whenever the bit is set.
fn num_to_bits<T: std::fmt::Binary>(num: T) -> Vec<bool> {
    let s = format!("{:04b}", num);
    s.chars()
        .map(|c| c == '1')
        .collect::<Vec<bool>>()
} 

/// Given a string representing a hexadecimal number,
/// where each character of the string is a hexadecimal digit representing 4 binary bits,
/// return a bitfield of the unsigned binary representation of that number,
/// msb at index 0
fn hex_to_bits<'a>(hex: &'a str) -> Vec<bool> {
    (0..hex.len())
        .map(|i| &hex[i..i+1])
        .map(|slice| u8::from_str_radix(slice, 16).unwrap())
        .flat_map(|num| num_to_bits(num))
        .collect::<Vec<bool>>()
}

fn hashes(seed: &str) -> Vec<String> {
    (0..128)
        .map(|i| format!("{}-{}", seed, i))
        .map(|plaintext| {
            let mut knot = Knot::new();
            knot.hash(Cursor::new(plaintext))
        })
        .collect()
}

fn bitcount_hash(hash: &str) -> u32 {
    let mut bitsum = 0;
    for j in 0..32 {
        let slice = &hash[j..j+1];
        let num = u32::from_str_radix(slice, 16).unwrap();
        bitsum += num.count_ones();
    }
    bitsum
}

fn count_hash_seed(s: &str) -> u32 {
    let mut bitsum = 0;
    for hash in hashes(&s) {
        bitsum += bitcount_hash(&hash);
    }
    bitsum
}

fn part_one() {
    let input = "jxqlasbh";
    println!("{}: {}", input, count_hash_seed(&input));
}

// --- Part Two ---
// Now, all the defragmenter needs to know is the number of regions. A region is a group of used squares that are all adjacent, not including diagonals. Every used square is in exactly one region: lone used squares form their own isolated regions, while several adjacent squares all count as a single region.

// In the example above, the following nine regions are visible, each marked with a distinct digit:

// 11.2.3..-->
// .1.2.3.4   
// ....5.6.   
// 7.8.55.9   
// .88.5...   
// 88..5..8   
// .8...8..   
// 88.8.88.-->
// |      |   
// V      V   
// Of particular interest is the region marked 8; while it does not appear contiguous in this small view, all of the squares marked 8 are connected when considering the whole 128x128 grid. In total, in this example, 1242 regions are present.

// How many regions are present given your key string?

fn make_grid(hash_seed: &str) -> Vec<Vec<bool>> {
    let mut grid = Vec::with_capacity(128);
    for hash in hashes(hash_seed) {
        grid.push(hex_to_bits(&hash));
    }
    grid
}

/// make a single scan through the grid
// At each position, if the cell is filled, look in each cardinal direction for adjacent clusters
// If at least one is found, merge this element and all clusters that it is touching into the 
// cluster with the lowest id that was found. 
// If none are found, then start a new cluster on this cell.

type ClusterId = i32;
#[derive(Debug)]
struct Loc(usize, usize);
type CGrid = Vec<Vec<CellState>>;
type CMap = HashMap<ClusterId, Vec<Loc>>;

#[derive(PartialEq, Eq, Debug, Clone)]
enum CellState {
    Unclaimed,
    Empty,
    Id(ClusterId)
}

struct Clusters {
    grid: CGrid,
    index: CMap,
    next_id: ClusterId
}

impl Clusters {
    fn new(size: u32) -> Self {
        let mut grid : Vec<Vec<CellState>> = Vec::new();
        for _ in 0..size {
            let mut row = vec![];
            for _ in 0..size {
                row.push(CellState::Unclaimed); 
            }
            grid.push(row);
        }
        Clusters { grid, index: HashMap::new(), next_id: 0 }
    }

    fn print_small(&self, window_size: usize) {
        for row in self.grid.iter().take(window_size) {
            println!("\n{}", row.iter().take(window_size).map(|c| match c {
                &CellState::Id(id) => format!("{:4}", id),
                &CellState::Empty => "   .".to_string(),
                &CellState::Unclaimed => "   ?".to_string()
            })
            .collect::<Vec<String>>()
            .join(" "));
        }
    }

    fn add_grid(&mut self, &Loc(i, j): &Loc, id: ClusterId) {
        self.grid[i][j] = CellState::Id(id);
    }
    fn new_cluster(&mut self, loc: Loc) {
        let id = self.next_id;
        self.next_id += 1;
        self.add_to_cluster(loc, id);
    }
    fn add_to_cluster(&mut self, loc: Loc, id: ClusterId) {
        self.add_grid(&loc, id);
        match self.index.entry(id) {
            Occupied(mut e) => { e.get_mut().push(loc); }
            Vacant(e) => { e.insert(vec![loc]); }
        }
    }
    fn set_empty(&mut self, Loc(i, j): Loc) {
        self.grid[i][j] = CellState::Empty;
    }
    fn state(&self, &Loc(i, j): &Loc) -> CellState {
        self.grid[i][j].clone()
    }
    fn merge_clusters(&mut self, dest: ClusterId, other: &ClusterId) {
        if dest == *other {
            return;
        }
        if let Some(mut locs) = self.index.remove(&other) {
            for loc in locs.iter() {
                self.add_grid(&loc, dest);
            }
            self.index.entry(dest)
                .and_modify(|f| f.append(&mut locs))
                .or_insert_with(|| locs );
        }
    }
}
fn print_small_grid(size: u32, occupied: &Vec<Vec<bool>>) {
    for row in occupied.iter().take(size as usize) {
        println!("\n{}", row.iter().take(size as usize).map(|c| match c {
            &true => "#",
            &false => ".",
        })
        .collect::<Vec<&str>>()
        .join(" "));
    }
}
/*
    This algorithm makes one pass through the grid, left to right, top to bottom.
    At each cell, if the cell is occupied, it checks all neighboring cells for any that 
    belong to a cluster. Then current cell and all of its cluster neighbors are merged into
    the lowest-id cluster that it finds.

    If the cell is occupied but has no neighbors that belong to cells, a new cluster is started.

*/
fn count_clusters(occupied: &Vec<Vec<bool>>) -> u32 {
    let size = 128;
    let mut clusters = Clusters::new(size);
    let len = clusters.grid.len();
    // print_small_grid(10, &occupied);
    for i in 0..len {
        let jlen = clusters.grid[i].len();
        for j in 0..jlen {
            let val  = clusters.state(&Loc(i, j));
            if occupied[i][j] {
                let mut adj_clusters = vec![];
                for o in [-1, 1].iter() {
                    let it = (i as i64) + *o;
                    let jt = (j as i64) + *o;

                    if it >= 0 && it < len as i64 {
                        let loc = Loc(it as usize, j);
                        if let CellState::Id(id) = clusters.state(&loc) {
                            adj_clusters.push(id);
                        }
                    }
                    if jt >= 0  && jt < jlen as i64 {
                        let loc = Loc(i, jt as usize);
                        if let CellState::Id(id) = clusters.state(&loc) {
                            adj_clusters.push(id);
                        }
                    }
                }

                if adj_clusters.len() > 0 {
                    let min = adj_clusters.iter().clone().min().unwrap();
                    for id in adj_clusters.iter() {
                        clusters.merge_clusters(*min, &id);
                    }
                    clusters.add_to_cluster(Loc(i, j), *min);
                } else {
                    clusters.new_cluster(Loc(i, j));
                }
            }
            else {
                clusters.set_empty(Loc(i, j))
            }
        }
    }
    // clusters.print_small(10);
    clusters.index.keys().len() as u32
}

fn part_two() {
    let grid = make_grid("jxqlasbh");
    let count = count_clusters(&grid);
    println!("14-2: {} clusters in {}", count, "jxqlasbh");
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use count_hash_seed;
    use hex_to_bits;
    use count_clusters;
    use make_grid;
    #[test]
    fn test_count_clusters() {
        assert_eq!(count_clusters(&make_grid("flqrgnkx")), 1242);
    }
    #[test]
    fn test_count_hash_seed() {
        assert_eq!(count_hash_seed("flqrgnkx"), 8108);
    }
    #[test]
    fn test_hex_to_bits() {
        for (expected_value, letter) in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"].iter().enumerate() {
            let actual = hex_to_bits(letter);
            let actual_binary_string = actual
                .iter()
                .map(|b| if *b { '1' } else { '0' }).collect::<String>();
            let actual_value = u8::from_str_radix(&actual_binary_string, 2).unwrap();
            assert_eq!(actual_value, expected_value as u8);
        }
    }

}
