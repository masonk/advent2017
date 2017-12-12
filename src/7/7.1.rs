// --- Day 7: Recursive Circus ---

// Wandering further through the circuits of the computer, you come upon a tower of programs that have gotten themselves into a bit of trouble. A recursive algorithm has gotten out of hand, and now they're balanced precariously in a large tower.

// One program at the bottom supports the entire tower. It's holding a large disc, and on the disc are balanced several more sub-towers. At the bottom of these sub-towers, standing on the bottom disc, are other programs, each holding their own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many programs stand simply keeping the disc below them balanced but with no disc of their own.

// You offer to help, but first you need to understand the structure of these towers. You ask each program to yell out their name, their weight, and (if they're holding a disc) the names of the programs immediately above them balancing on that disc. You write this information down (your puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the time you're done, you're not sure which program gave which information.

// For example, if your list is the following:

// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)
// ...then you would be able to recreate the structure of the towers that looks like this:

//                 gyxo
//               /     
//          ugml - ebii
//        /      \     
//       |         jptl
//       |        
//       |         pbga
//      /        /
// tknk --- padx - havc
//      \        \
//       |         qoyq
//       |             
//       |         ktlj
//        \      /     
//          fwft - cntj
//               \     
//                 xhth
// In this example, tknk is at the bottom of the tower (the bottom program), and is holding up ugml, padx, and fwft. Those programs are, in turn, holding up other programs; in this example, none of those programs are holding up any other programs, and are all the tops of their own towers. (The actual tower balancing in front of you is much larger.)

// Before you're ready to help them, you need to make sure your information is correct. What is the name of the bottom program?

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
extern crate regex;
use regex::Regex;


#[derive(Debug)]
struct Node {
    key: String,
    weight: i32,
    children: Vec<String>,
    parents: Vec<String>
}

fn line_to_node(line: String) -> Result<Node, String> {
    {
        let split = line.split(" ").collect::<Vec<&str>>();
        if split.len() > 1 {
            let parens = Regex::new(r"[\(\)]").unwrap();

            if let Ok(weight) = parens.replace_all(split[1], "").parse() {
                let commas = Regex::new(",").unwrap();
                let children = if split.len() > 3 {
                    split[3..].iter().map(|c| commas.replace(c, "").into_owned()).collect()
                } else {
                    vec![]
                };
                    
                return Ok(Node {
                    key: split[0].to_string(),
                    weight,
                    children,
                    parents: vec![]
                });
            }
        }
    }
    Err(line)
}

fn populate_parents(graph: &mut HashMap<String, Node>) {
    let g_ptr = graph as *mut HashMap<String, Node>;

    for (key, node) in graph.iter() {
        for ref childkey in node.children.iter() {
            unsafe { // I really do want to mutate this graph while iterating over it
                if let Some(ref mut child) = (*g_ptr).get_mut(*childkey) {
                    child.parents.push(key.to_string());
                }
            }
        }
    }
}

fn parse_graph(reader: &mut BufRead) -> HashMap<String, Node> {
    let mut graph = HashMap::<String, Node>::new();

    for line in reader.lines().map(|l| l.expect("Couldnt parse a line")) {
        let maybe_node = line_to_node(line);
        match maybe_node {
            Ok(n) => { graph.insert(n.key.clone(), n); },
            Err(m) => { println!("Couldn't parse {}", m)},
        }
    }
    graph
}

fn find_root<'a>(entry: &str, graph: &'a HashMap<String, Node>) -> &'a String {
    // start from any node in the graph recursively follow its first parent
    // until we encounter a node with no parents. By the problem description there is only one such node.
    let mut current = graph.get(entry).expect("tried to index a node that isn't in the graph");

    while current.parents.len() > 0 {   
        let next = graph.get(&current.parents[0]).expect("tried to get a node that isn't in the graph");
        println!("{} -> {}", current.key, next.key);
        current = next;
    }
    return &current.key;
}

// --- Part Two ---

// The programs explain the situation: they can't get down. Rather, they could get down, if they weren't expending all of their energy trying to keep the tower balanced. Apparently, one program has the wrong weight, and until it's fixed, they're stuck here.

// For any program holding a disc, each program standing on that disc forms a sub-tower. Each of those sub-towers are supposed to be the same weight, or the disc itself isn't balanced. The weight of a tower is the sum of the weights of the programs in that tower.

// In the example above, this means that for ugml's disc to be balanced, gyxo, ebii, and jptl must all have the same weight, and they do: 61.

// However, for tknk to be balanced, each of the programs standing on its disc and all programs above it must each match. This means that the following sums must all be the same:

// ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
// padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
// fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
// As you can see, tknk's disc is unbalanced: ugml's stack is heavier than the other two. Even though the nodes above ugml are balanced, ugml itself is too heavy: it needs to be 8 units lighter for its stack to weigh 243 and keep the towers balanced. If this change were made, its weight would be 60.

// Given that exactly one program is the wrong weight, what would its weight need to be to balance the entire tower?
fn find_unbalanced_disc<'a>(root: &str, graph: &'a HashMap<String, Node>) -> &'a String {
    // Invariant: The bad node is always reachable from 'current'.
    // If no children have different weights, or if there are no children, then the current node is the bad one (by our invariant).

    // If a node has only two children and the two children have different weights, then
    // only one of the children can have unbalanced children. If both had unbalanced grandchildren, 
    // then at least two nodes would need to change to balance the towers, impossible by the p.d.

    let mut current = graph.get(root).expect("tried to index a node that isn't in the graph");
    let mut good = 0; // the weight that children should be in this branch

    loop {
        println!("analyzing {:?}", &current.key);
        for ref c in current.children.iter() {
            let child = graph.get(*c).unwrap();
            println!("{}: {}", &child.key, weight_inclusive(c, &graph));
        }
        if current.children.len() == 0 {
            println!("{} has no children, it must be the culprit", &current.key);
            return &current.key;
        }
        
        if current.children.len() == 1 {
            panic!("Shouldn't have a node with only 1 child");
        }

        // if all children have the same weight, then current is the problem
        
        let first_child = graph.get(&current.children[0]).unwrap();
        let first_child_weight = weight_inclusive(&first_child.key, &graph);
        let second_child = graph.get(&current.children[1]).unwrap();
        let second_child_weight = weight_inclusive(&second_child.key, &graph);

        let mut weight_iter = current.children.iter().map(|c| weight_inclusive(&c, &graph));
        if allsame(&mut weight_iter) {
            let cweight = current.weight;
            let badweight = weight_inclusive(&current.key, &graph);
            let diff = good - badweight;

            println!("{}'s children all have the same weight, so {} is the bad node. Its inclusive weight is {}, and its sibling's inclusive weight is {}. {} + {} = {}", &current.key, &current.key, badweight, good, cweight, diff, cweight + diff);
            return &current.key;
        }

        // if there are three or more children, then the odd man out contains the problem
        // and we know exactly how much weight our single problem node is different from the good weight
        if current.children.len() > 2 {


            if first_child_weight == second_child_weight {
                good = first_child_weight;
                for ref c in current.children[2..].iter() {
                    let cw = weight_inclusive(c, &graph);
                    if cw != good {
                        current = graph.get(*c).unwrap();
                        break;
                    }
                }
            } else {
                if second_child_weight == weight_inclusive(&current.children[2], &graph) {
                    good = second_child_weight;
                    current = first_child;
                }
                else {
                    good = first_child_weight;
                    current = second_child;
                }
            }
            println!("{} was the one strange child", &current.key);
        } else {
            let mut first_child_weight_iter = first_child.children.iter().map(|c| weight_inclusive(&c, &graph));
            let mut second_child_weight_iter = second_child.children.iter().map(|c| weight_inclusive(&c, &graph));
            if !allsame(&mut first_child_weight_iter) {
                current = first_child;
            } else if !allsame(&mut second_child_weight_iter) {
                current = second_child;
            } else {
                panic!("It could be either {} or {}", &first_child.key, &second_child.key);
            }
            println!("There were two mismatched children, but only {} had mismatched grandchildren", &current.key);
        }
    }
}

fn allsame<I>(i: &mut I) -> bool
    where I: Iterator, I::Item: Eq {
    
    let first = match i.next() {
        Some(f) => f,
        _ => return true,
    };

    let mut same = true;
    for c in i {
        if first != c {
            same = false;
            break;
        }
    }
    return same;
}

fn weight_inclusive(entry: &str, graph: &HashMap<String, Node>) -> i32 {
    
    let node = graph.get(entry).expect("tried to index a node that isn't in the graph");
    let mut sum = node.weight;
    for ref child in node.children.iter() {
        sum += weight_inclusive(child, &graph);
    }
    sum
}

fn main() {
    let fname = "src/7/data";
    
    let input = File::open(fname)
        .expect("Couldn't open file");
    
    let mut reader = BufReader::new(&input);
    let mut graph = parse_graph(&mut reader);
    populate_parents(&mut graph);

    let root = find_root("vtryv", &graph);
    let unbalanced = find_unbalanced_disc(root, &graph);
    println!("unbalanced was {}", unbalanced);
}