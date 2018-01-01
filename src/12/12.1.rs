// --- Day 12: Digital Plumber ---

// Walking along the memory banks of the stream, you find a small village that is experiencing a little confusion: some programs can't comm unicate with each other.

// Programs in this village communicate using a fixed system of pipes. Messages are passed between programs using these pipes, but most programs aren't connected to each other directly. Instead, programs pass messages between each other until the message reaches the intended recipient.

// For some reason, though, some of these messages aren't ever reaching their intended recipient, and the programs suspect that some pipes are missing. They would like you to investigate.

// You walk through the village and record the ID of each program and the IDs with which it can communicate directly (your puzzle input). Each program has one or more programs with which it can communicate, and these pipes are bidirectional; if 8 says it can communicate with 11, then 11 will say it can communicate with 8.

// You need to figure out how many programs are in the group that contains program ID 0.

// For example, suppose you go door-to-door like a travelling salesman and record the following list:

// 0 <-> 2
// 1 <-> 1
// 2 <-> 0, 3, 4
// 3 <-> 2, 4
// 4 <-> 2, 3, 6
// 5 <-> 6
// 6 <-> 4, 5
// In this example, the following programs are in the group that contains program ID 0:

// Program 0 by definition.
// Program 2, directly connected to program 0.
// Program 3 via program 2.
// Program 4 via program 2.
// Program 5 via programs 6, then 4, then 2.
// Program 6 via programs 4, then 2.

// Therefore, a total of 6 programs are in this group; all but program 1, which has a pipe that connects it to itself.
 #![feature(entry_and_modify)]
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
 
type PipeHash = HashMap<i32, HashSet<i32>>;

fn parse_pipemap<T: Read>(stream: T) -> PipeHash {
    let buf = BufReader::new(stream);
    let mut map : PipeHash = HashMap::new();

    for l in buf.lines() {
        let line = l.unwrap();
        let split = line.split(" <-> ").collect::<Vec<&str>>();
        let lhs = split[0].parse::<i32>().unwrap();
        let rhs = split[1];
        let connections = rhs.split(", ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        map.entry(lhs)
            .and_modify(|set| for con in connections.iter() {
                set.insert(*con);
            })
            .or_insert_with(|| {
                let mut set : HashSet<i32> = HashSet::new();
                for con in connections {
                    set.insert(con);
                }
                set
            });
    }
    map
}
fn find_connections(start: i32, map: &PipeHash) -> Result<HashSet<i32>, String>  {
    let mut connections = HashSet::new();
    _find_connections(start, &map, &mut connections)?;
    Ok(connections)
}
fn _find_connections(start: i32, map: &PipeHash, connections: &mut HashSet<i32>) -> Result<(), String> {
    let children = map.get(&start).ok_or(format!("Couldn't find start {}", start))?;
    for c in children {
        if connections.contains(c) { continue }
        connections.insert(*c);
        _find_connections(*c, map, connections);
    }
    Ok(())
}

// --- Part Two ---
// There are more programs than just the ones in the group containing program ID 0. The rest of them have no way of reaching that group, and still might have no way of reaching each other.

// A group is a collection of programs that can all communicate via pipes either directly or indirectly. The programs you identified just a moment ago are all part of the same group. Now, they would like you to determine the total number of groups.

// In the example above, there were 2 groups: one consisting of programs 0,2,3,4,5,6, and the other consisting solely of program 1.

// How many groups are there in total?

fn one_key(map: &PipeHash) -> Option<i32> {
    for (k, _) in map {
        return Some(*k);
    }
    None
}
fn count_groups(mut map: PipeHash) -> u32 {
    let mut count = 0u32;
    while let Some(next) = one_key(&map) {
        count += 1;
        let group = find_connections(next, &map).unwrap();
        for ref k in group {
            map.remove(k);
        }
    }
    count
}

fn main() {
    let f = File::open("src/12/data").unwrap();
    let hash = parse_pipemap(f);
    println!("{:?}", hash);
    let connections = find_connections(0, &hash).unwrap();
    let count = count_groups(hash);
    println!("cons from 0: {:?}. (Length: {})", connections, connections.len());
    println!("found {} separate groups", count)

}