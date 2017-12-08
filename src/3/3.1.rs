// --- Day 3: Spiral Memory ---

// You come across an experimental new kind of memory stored on an infinite two-dimensional grid.

// Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and then counting up while spiraling outward. For example, the first few squares are allocated like this:

// 17  16  15  14  13
// 18   5   4   3  12
// 19   6   1   2  11
// 20   7   8   9  10
// 21  22  23---> ...
// While this is very space-efficient (no squares are skipped), requested data must be carried back to square 1 (the location of the only access port for this memory system) by programs that can only move up, down, left, or right. They always take the shortest path: the Manhattan Distance between the location of the data and square 1.

// For example:

// Data from square 1 is carried 0 steps, since it's at the access port.
// Data from square 12 is carried 3 steps, such as: down, left, left.
// Data from square 23 is carried only 2 steps: up twice.
// Data from square 1024 must be carried 31 steps.
// How many steps are required to carry the data from the square identified in your puzzle input all the way to the access port?

// Your puzzle input is 368078.
// -----------------------
// Note that the lower right diagonal from 1 is (1 + 2l)^2 
// for some non-negative integer l. That is, each box surrounding 1 
// contains (1 + 2l) ^ 2 numbers within it.

//  maxindex(l) = (1 + 2l) ^ 2

// For a given address a, there is some number l which is the smallest number
// for which (1 + 2l) ^ 2 >= a.

// Define this l as the "layer" of a.

// 1 + 4l^2 + 4l >= a

// Use the quadratic equation to solve for exactly the case where 

// 4l^2 + 4l + 1 = a

// 4l^2 + 4l + 1-a = 0

// and round up to find the layer of a.

// layer(a) = ceil(-4/8 +- sqrt(16 - 16(1 - a))/8)
//          = ceil(-1/2 +- sqrt(16 - (16 - 16a))/8)
//          = ceil(-1/2 +- sqrt(16a)/8)

// layer(252340) = 251
// layer(25) = -1/2 + 20/8 = 2 (checks out)
// layer(26) = ceil(2.04) = 3 (checks out)

// For an address a in a layer l, the Manhattan distance between a and 1 
// is somewhere between l and 2l. It is l if the address lies in a cardinal 
// direction from 1, e.g., straight up or straight left, and it is 2l if the
// address lies on a diagonal.

// Another way of think about this is that the distance is l + x, where x
// is the number of steps it takes to get to a cardinal direction of layer l.

// A layer's size is the number of items that it contains 
// which no smaller layer contains, which is exactly 
// size(l) = (1 + 2l)^2 - (1 + 2(l - 1))^2
//         = (1 + 2l)^2 - (2l - 1)^2
//         = (1 + 4l^2 + 4l) - (4l^2 -4l + 1)
//         = 2 + 4l + 4l
//         = 8l

// (Since l is an integer we know that size(a) will always be divisible by 4,
// this comes in handy below).

// An address's layer offset is maxindex(layer(a)) - a, which is how many backwards
// (clockwise) around the spiral we have to count to get from the largest 
// possible address in l to the actual address a.

// If we have to count 0 backwards, a == maxindex(layer(a)), meaning it's on the
// lower right diagonal. Then the distance is exactly 2i.

// We want to know how far away an offset is from its closest diagonal. Every
// jump away from a diagonal is one jump closer to 1.

// The closest_diagonal_offset of a is the number of jumps that a has to make
// to get to the closest of these diagonal points in the layer of a.

// Therefore:

// manhattan(a) = 2(layer(a)) - closest_diagonal_offset(a)

use std::cmp;

fn layer(a: u64) -> u64 {
    let term1 = ((16 * a) as f64).sqrt() / 8.0 - 0.5;
    term1.ceil() as u64
}

fn size(l: u64) -> u64 { 8 * l }

fn maxindex(l: u64) -> u64 { (1 + 2 * l).pow(2) }

fn offset(a: u64) -> u64 { maxindex(layer(a)) - a }

fn closest_diagonal_offset(a: u64) -> u64 {
    if a == 1 {
        0
    }
    else {
        let modulus = size(layer(a)) / 4;
        let m = offset(a) % modulus;
        cmp::min(m, modulus - m)
    }

}

fn manhattan(a: u64) -> u64 {
    if a == 1 {
        0
    } else {
        2*layer(a) - closest_diagonal_offset(a)
    }
}

fn main() {
    println!("manhattan({}): {}", 368078, manhattan(368078));
}



 

