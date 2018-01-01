// --- Day 13: Packet Scanners ---
// You need to cross a vast firewall. The firewall consists of several layers, each with a security scanner that moves back and forth across the layer. To succeed, you must not be detected by a scanner.

// By studying the firewall briefly, you are able to record (in your puzzle input) the depth of each layer and the range of the scanning area for the scanner within it, written as depth: range. Each layer has a thickness of exactly 1. A layer at depth 0 begins immediately inside the firewall; a layer at depth 1 would start immediately after that.

// For example, suppose you've recorded the following:

// 0: 3
// 1: 2
// 4: 4
// 6: 4
// This means that there is a layer immediately inside the firewall (with range 3), a second layer immediately after that (with range 2), a third layer which begins at depth 4 (with range 4), and a fourth layer which begins at depth 6 (also with range 4). Visually, it might look like this:

//  0   1   2   3   4   5   6
// [ ] [ ] ... ... [ ] ... [ ]
// [ ] [ ]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]
// Within each layer, a security scanner moves back and forth within its range. Each security scanner starts at the top and moves down until it reaches the bottom, then moves up until it reaches the top, and repeats. A security scanner takes one picosecond to move one step. Drawing scanners as S, the first few picoseconds look like this:


// Picosecond 0:
//  0   1   2   3   4   5   6
// [S] [S] ... ... [S] ... [S]
// [ ] [ ]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]

// Picosecond 1:
//  0   1   2   3   4   5   6
// [ ] [ ] ... ... [ ] ... [ ]
// [S] [S]         [S]     [S]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]

// Picosecond 2:
//  0   1   2   3   4   5   6
// [ ] [S] ... ... [ ] ... [ ]
// [ ] [ ]         [ ]     [ ]
// [S]             [S]     [S]
//                 [ ]     [ ]

// Picosecond 3:
//  0   1   2   3   4   5   6
// [ ] [ ] ... ... [ ] ... [ ]
// [S] [S]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [S]     [S]
// Your plan is to hitch a ride on a packet about to move through the firewall. The packet will travel along the top of each layer, and it moves at one layer per picosecond. Each picosecond, the packet moves one layer forward (its first move takes it into layer 0), and then the scanners move one step. If there is a scanner at the top of the layer as your packet enters it, you are caught. (If a scanner moves into the top of its layer while you are there, you are not caught: it doesn't have time to notice you before you leave.) If you were to do this in the configuration above, marking your current position with parentheses, your passage through the firewall would look like this:

// Initial state:
//  0   1   2   3   4   5   6
// [S] [S] ... ... [S] ... [S]
// [ ] [ ]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]

// Picosecond 0:
//  0   1   2   3   4   5   6
// (S) [S] ... ... [S] ... [S]
// [ ] [ ]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]

//  0   1   2   3   4   5   6
// ( ) [ ] ... ... [ ] ... [ ]
// [S] [S]         [S]     [S]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]


// Picosecond 1:
//  0   1   2   3   4   5   6
// [ ] ( ) ... ... [ ] ... [ ]
// [S] [S]         [S]     [S]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]

//  0   1   2   3   4   5   6
// [ ] (S) ... ... [ ] ... [ ]
// [ ] [ ]         [ ]     [ ]
// [S]             [S]     [S]
//                 [ ]     [ ]


// Picosecond 2:
//  0   1   2   3   4   5   6
// [ ] [S] (.) ... [ ] ... [ ]
// [ ] [ ]         [ ]     [ ]
// [S]             [S]     [S]
//                 [ ]     [ ]

//  0   1   2   3   4   5   6
// [ ] [ ] (.) ... [ ] ... [ ]
// [S] [S]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [S]     [S]


// Picosecond 3:
//  0   1   2   3   4   5   6
// [ ] [ ] ... (.) [ ] ... [ ]
// [S] [S]         [ ]     [ ]
// [ ]             [ ]     [ ]
//                 [S]     [S]

//  0   1   2   3   4   5   6
// [S] [S] ... (.) [ ] ... [ ]
// [ ] [ ]         [ ]     [ ]
// [ ]             [S]     [S]
//                 [ ]     [ ]


// Picosecond 4:
//  0   1   2   3   4   5   6
// [S] [S] ... ... ( ) ... [ ]
// [ ] [ ]         [ ]     [ ]
// [ ]             [S]     [S]
//                 [ ]     [ ]

//  0   1   2   3   4   5   6
// [ ] [ ] ... ... ( ) ... [ ]
// [S] [S]         [S]     [S]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]


// Picosecond 5:
//  0   1   2   3   4   5   6
// [ ] [ ] ... ... [ ] (.) [ ]
// [S] [S]         [S]     [S]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]

//  0   1   2   3   4   5   6
// [ ] [S] ... ... [S] (.) [S]
// [ ] [ ]         [ ]     [ ]
// [S]             [ ]     [ ]
//                 [ ]     [ ]


// Picosecond 6:
//  0   1   2   3   4   5   6
// [ ] [S] ... ... [S] ... (S)
// [ ] [ ]         [ ]     [ ]
// [S]             [ ]     [ ]
//                 [ ]     [ ]

//  0   1   2   3   4   5   6
// [ ] [ ] ... ... [ ] ... ( )
// [S] [S]         [S]     [S]
// [ ]             [ ]     [ ]
//                 [ ]     [ ]
// In this situation, you are caught in layers 0 and 6, because your packet entered the layer when its scanner was at the top when you entered it. You are not caught in layer 1, since the scanner moved into the top of the layer once you were already there.

// The severity of getting caught on a layer is equal to its depth multiplied by its range. (Ignore layers in which you do not get caught.) The severity of the whole trip is the sum of these values. In the example above, the trip severity is 0*3 + 6*4 = 24.

// Given the details of the firewall you've recorded, if you leave immediately, what is the severity of your whole trip?

// Now, you need to pass through the firewall without being caught - easier said than done.

// You can't control the speed of the packet, but you can delay it any number of picoseconds. For each picosecond you delay the packet before beginning your trip, all security scanners move one step. You're not in the firewall during this time; you don't enter layer 0 until you stop delaying the packet.

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
#[derive(Debug)]
struct Scanners {
    max_depth: i32,
    max_range: i32,
    scanners: HashMap<i32, i32>
}

impl Scanners {
    // The position of the scanner at depth d after it has run for time t.
    // None if there no scanner at depth d
    fn pos(&self, d: &i32, t: &i32) -> Option<i32> {  
        if let Some(r) = self.range(&d) {
            if *t < 0 {
                return Some(0);
            }

            let mi = r - 1; /* max index of the scanner range */
            let unique_positions = r * 2 - 2; /* how many different states the scanner can be in. Whenever the scanner is at an end, it can only be turning around. Whenever a scanner is in a middle position, it could be going back or forward*/
            let pos = *t % unique_positions;
            if pos < *r {
                return Some(pos)
            } else {
                return Some(mi - (pos % mi));
            }
        }
        None
    }
    fn collides(&self, d: &i32, t: &i32) -> bool {
        if let Some(r) = self.range(d) {
            // There is a collision iff t % r * 2 - 2 == 0
            return *t % (*r * 2 - 2) == 0;
        }
        false
    }
    fn range(&self, d: &i32) -> Option<&i32> {
        self.scanners.get(d)
    }
}

fn get_scanners(f: File) -> Scanners {
    let buf = BufReader::new(f);
    let mut scanners = HashMap::new();
    let mut max_depth = 0;
    let mut max_range = 0;
    for line in buf.lines() {
        let split = line.expect("io error")
            .split(": ")
            .map(|s| s.parse::<i32>().expect("parse int err"))
            .collect::<Vec<i32>>();

        scanners.insert(split[0], split[1]);
        max_depth = if split[0] > max_depth { split[0] } else { max_depth };
        max_range = if split[1] > max_range { split[1] } else { max_range };
    }

    Scanners { max_range, max_depth, scanners }
}

// Print the packet depth and show scanner positions after they've run for t picoseconds
// fn debug_print_scanners(packet_d: &i32, t: &i32, scanners: &Scanners) {
//     println!("Picosecond {} ---", t);
//     println!("{}", (0..scanners.max_depth + 1).map(|i| format!(" {} ", i)).collect::<Vec<String>>().join(" "));
//     for i in 0..scanners.max_range {
//         let mut cells = vec![];
//         for d in 0..scanners.max_depth + 1 {
//             let pos = scanners.pos(&d, &t);
//             let cell = match pos {
//                 Some(p) => {
//                     if i == 0 && p == 0 && *packet_d == d  {
//                         "[!]"
//                     }
//                     else if i == 0 && *packet_d == d  {
//                         "[*]"
//                     }
//                     else if i == p {
//                         "[S]"
//                     } else if i >= *scanners.range(&d).unwrap() {
//                         "   "
//                     } else {
//                         "[ ]"
//                     }
//                 },
//                 _ => 
//                     if i == 0 && *packet_d == d  {
//                         " * "
//                     }
//                     else if i == 0 { 
//                         "..." 
//                     } else {
//                         "   "
//                     }
//             };
//             cells.push(cell);
//         }
//         println!("{}\n", &cells.join(" "));
//     }
// }

/* Advent13-1 the total severity of starting at a given offset */
fn severity(offset: &i32, scanners: &Scanners) -> i32 {
    let mut severity = 0;
    let mut d : i32 = 0;
    while d <= scanners.max_depth {
        let scanner_time = d + offset;
        if let Some(pos) =  scanners.pos(&d, &scanner_time) {
            if pos == 0 {
                let r = scanners.range(&d).unwrap();
                // println!("Hit layer {} and got severity {}", d, r*d);
                severity += *r * d;
            }
        }
        // debug_print_scanners(&d, &scanner_time, &scanners);
        d += 1;
    }
    severity
}

// /* Advent13-2 the total number of detections starting at a given offset */
// fn detections(offset: &i32, scanners: &Scanners) -> i32 {
//     let mut detections = 0;
//     let mut d : i32 = 0;
//     while d <= scanners.max_depth {
//         let scanner_time = d + offset;
//         if scanners.collides(&d, &scanner_time) {
//             detections += 1;
//         }
//         // debug_print_scanners(&d, &scanner_time, &scanners);
//         d += 1;
//     }
//     detections
// }

/* Advent13-2 the total number of detections starting at a given offset */
fn detected(offset: &i32, scanners: &Scanners) -> bool {
    let mut d : i32 = 0;
    while d <= scanners.max_depth {
        let scanner_time = d + offset;
        if scanners.collides(&d, &scanner_time) {
            return true;
        }
        d += 1;
    }
    false
}

fn main() {
    let fname = "src/13/data";
    // let fname = "src/13/testdata";
    let f = File::open(fname).expect(&format!("Couldn't open {}", fname));
    let scanners = get_scanners(f);
    
    println!("Advent 13-1: severity {} at offset 0.", severity(&0, &scanners));

    let mut offset = 0;
    while detected(&offset, &scanners) {
        offset += 1;
    }
    println!("Advent 13-2: 0 detections at offset {}.", offset);
}