// -- Day 8: I Heard You Like Registers ---

// You receive a signal directly from the CPU. Because of your recent assistance with jump instructions, it would like you to compute the result of a series of unusual register instructions.

// Each instruction consists of several parts: the register to modify, whether to increase or decrease that register's value, the amount by which to increase or decrease it, and a condition. If the condition fails, skip the instruction without modifying the register. The registers all start at 0. The instructions look like this:

// b inc 5 if a > 1
// a inc 1 if b < 5
// c dec -10 if a >= 1
// c inc -20 if c == 10
// These instructions would be processed as follows:

// Because a starts at 0, it is not greater than 1, and so b is not modified.
// a is increased by 1 (to 1) because b is less than 5 (it is 0).
// c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
// c is increased by -20 (to -10) because c is equal to 10.
// After this process, the largest value in any register is 1.

// You might also encounter <= (less than or equal to) or != (not equal to). However, the CPU doesn't have the bandwidth to tell you what all the registers are named, and leaves that to you to determine.

// What is the largest value in any register after completing the instructions in your puzzle input?

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

type CmpFn = Box<Fn(&i32, &i32) -> bool>;
type Ops<'a> = HashMap<&'a str, CmpFn>;

struct Condition<'a, 'b> {
    subject: &'a str,
    cmp: &'b Box<Fn(&i32, &i32) -> bool>,
    value: i32,
}

impl<'a, 'b> fmt::Debug for Condition<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Condition {{ subject: {}, value: {} }}", self.subject, self.value)
    }
}

#[derive(Debug)]
struct Op<'a, 'b> {
    subject: &'a str,
    addend: i32,
    condition: Condition<'a, 'b>
}

fn parse_op<'a, 'b>(line: &'a str, ops: &'b Ops) -> Result<Op<'a, 'b>, String> {
    let split = line.split(" ").collect::<Vec<&str>>();
    if split.len() < 7 {
        return Err(format!("split {} into fewer than 7 parts", line));
    }

    let subject = split[0];
    let mut addend = split[2].parse::<i32>().map_err(|err| err.to_string())?;
    if split[1] == "dec" {
        addend *= -1;
    }

    let cond_subj = split[4];
    let cmp = ops.get(split[5])
        .ok_or_else(|| format!("Didn't have an op for {}", split[5]))?;

    let value = split[6].parse::<i32>().map_err(|e| e.to_string())?;

    Ok(Op {
        subject,
        addend,
        condition: Condition {
            subject: cond_subj,
            cmp,
            value
        }
    })
}

fn process_instructions(reader: &mut BufRead, ops: &Ops, registers: &mut HashMap<String, i32>) -> i32 {
    let mut max = 0;
    for line in reader.lines().map(|l| l.expect("Couldnt parse a line")) {
        if let Ok(op) = parse_op(&line, &ops) {
            let should : bool;
            {
                let cond_value = registers.entry(String::from(op.condition.subject)).or_insert(0);
                should = (op.condition.cmp)(cond_value, &op.condition.value);
            }

            if should {
                let subj_value = registers.entry(String::from(op.subject)).or_insert(0);
                *subj_value += op.addend;
                if *subj_value > max { 
                    max = *subj_value 
                }
            }
        } else {
            println!("Couldn't decode line {}", line);
        }
    }
    max
}

fn main () {
    let fname = "src/8/data";
    
    let mut op_map : Ops = HashMap::new();
    op_map.insert("==", Box::new(|&a, &b| a == b));
    op_map.insert("!=", Box::new(|&a, &b| a != b));
    op_map.insert(">=", Box::new(|&a, &b| a >= b));
    op_map.insert("<=", Box::new(|&a, &b| a <= b));
    op_map.insert(">", Box::new(|&a, &b| a > b));
    op_map.insert("<", Box::new(|&a, &b| a < b));

    
    let input = File::open(fname)
        .expect("Couldn't open file");
    
    let mut reader = BufReader::new(&input);
    let mut registers = HashMap::new();
    let max = process_instructions(&mut reader, &op_map, &mut registers);
    process_instructions(&mut reader, &op_map, &mut registers);
    let mut f_max = 0;

    for (_, v) in registers.iter() {
        if *v > f_max {
            f_max = *v;
        }
    }
    println!("high water mark: {}, final max: {}", max, f_max);
}