use std::collections::HashMap;
use std::fmt;

type RegAdr = char;
type RegisterSize = i64;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Val {
    Addr(RegAdr),
    Lit(RegisterSize),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inst {
    Snd(Val),
    Set(RegAdr, Val),
    Add(RegAdr, Val),
    Mul(RegAdr, Val),
    Mod(RegAdr, Val),
    Rcv(RegAdr),
    Jgz(Val, Val),
}

use self::Inst::*;
use self::Val::*;

#[derive(Clone)]
pub struct DuetState {
    cursor: usize,
    sound: Option<RegisterSize>,
    instrs: Vec<Inst>,
    registers: HashMap<char, RegisterSize>,
}

impl fmt::Debug for DuetState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DuetState {{ cursor: {}, sound: {:?}, next_instr: {:?}, registers: {:?} }}",
            self.cursor, self.sound, self.instrs[self.cursor], self.registers
        )
    }
}
impl DuetState {
    pub fn new(instrs: Vec<Inst>) -> Self {
        DuetState {
            sound: None,
            cursor: 0,
            instrs,
            registers: HashMap::new(),
        }
    }

    pub fn play_until_recovery(&mut self) -> RegisterSize {
        loop {
            if self.cursor > self.instrs.len() {
                break;
            }
            {
                let inst = &self.instrs[self.cursor];
                if let &Rcv(v) = inst {
                    if self.resolve(&Addr(v)) != 0 {
                        // presumably, if there's no sound at the end of the sequence, that's a code error
                        // (Because they did not define that behavior in the problem statement)
                        return self.sound.unwrap();
                    }
                }
            }
            self.do_next();
        }
        self.sound.unwrap()
    }
    fn do_next(&mut self) {
        let inst = &self.instrs[self.cursor];
        let mut jmp = 1;
        println!("Before: {:?}", self);
        match *inst {
            Snd(ref v) => {
                self.sound = Some(self.resolve(v));
            }
            Set(addr, ref v) => {
                let val = self.resolve(v);
                self.registers.insert(addr, val);
            }
            Add(addr, ref v) => {
                let val = self.resolve(v);
                let mut entry = self.registers.entry(addr).or_insert(0);
                *entry += val;
            }
            Mul(addr, ref v) => {
                let val = self.resolve(v);
                let mut entry = self.registers.entry(addr).or_insert(0);
                *entry *= val;
            }
            Mod(addr, ref v) => {
                let val = self.resolve(v);
                let mut reg = self.registers.entry(addr).or_insert(0);
                *reg = *reg % val;
            }
            Rcv(ref v) => {
                // Don't know what "recovering" a sound is supposed to do. Maybe part 2.
            }
            Jgz(ref x, ref y) => {
                if self.resolve(x) > 0 {
                    jmp = self.resolve(y);
                }
            }
        }
        self.cursor = ((self.cursor as i64) + (jmp as i64)) as usize;
        // println!("After: {:?}", self);
    }

    fn resolve(&self, val: &Val) -> RegisterSize {
        match val {
            &Val::Lit(v) => v,
            &Val::Addr(ref addr) => {
                if let Some(v) = self.registers.get(addr) {
                    *v
                } else {
                    0
                }
            }
        }
    }
}

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::io::Write;
use std::fs::File;

use std::collections::VecDeque;
struct CommunicatingDuet {
    send_count: usize,
    done: bool,
    cursor: usize,
    instrs: Vec<Inst>,
    registers: HashMap<char, RegisterSize>,
    pid: i64,
    buf: VecDeque<RegisterSize>,
    blocked: bool,
    stopped: bool,
}
impl fmt::Debug for CommunicatingDuet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DuetState {{ next_instr: {:?}, registers: {:?}, send_count: {} }}",
            self.instrs[self.cursor], self.registers, self.send_count
        )
    }
}
impl CommunicatingDuet {
    fn new(pid: i64, instrs: Vec<Inst>) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', pid);
        CommunicatingDuet {
            pid,
            instrs,
            registers,
            send_count: 0,
            cursor: 0,
            done: false,
            buf: VecDeque::new(),
            blocked: false,
            stopped: false,
        }
    }
    fn stuck(&self) -> i32 {
        if self.blocked || self.stopped {
            return 1;
        }
        0
    }
    fn do_next(&mut self) -> Option<RegisterSize> {
        if self.stopped {
            return None;
        }
        let inst = self.instrs[self.cursor].clone();
        let mut jmp = 1;
        let mut ret = None;
        match inst {
            Snd(ref v) => {
                self.send_count += 1;
                let v = self.resolve(v);

                ret = Some(v);
            }
            Rcv(a) => {
                if let Some(v) = self.buf.pop_front() {
                    self.blocked = false;
                    self.registers.insert(a, v);
                } else {
                    self.blocked = true;
                    jmp = 0;
                }
            }
            Set(addr, ref v) => {
                let val = self.resolve(v);
                self.registers.insert(addr, val);
            }
            Add(addr, ref v) => {
                let val = self.resolve(v);
                let mut entry = self.registers.entry(addr).or_insert(0);
                *entry += val;
            }
            Mul(addr, ref v) => {
                let val = self.resolve(v);
                let mut entry = self.registers.entry(addr).or_insert(0);
                *entry *= val;
            }
            Mod(addr, ref v) => {
                let val = self.resolve(v);
                let mut reg = self.registers.entry(addr).or_insert(0);
                *reg = *reg % val;
            }

            Jgz(ref x, ref y) => {
                if self.resolve(x) > 0 {
                    jmp = self.resolve(y);
                }
            }
        }
        self.cursor = ((self.cursor as i64) + (jmp as i64)) as usize;
        if self.cursor >= self.instrs.len() {
            self.stopped = true;
        }
        ret
    }

    fn resolve(&self, val: &Val) -> RegisterSize {
        match val {
            &Val::Lit(v) => v,
            &Val::Addr(ref addr) => {
                if let Some(v) = self.registers.get(addr) {
                    *v
                } else {
                    0
                }
            }
        }
    }
}

pub struct DuetPair {
    a: CommunicatingDuet,
    b: CommunicatingDuet,
}

impl DuetPair {
    pub fn play(instrs: Vec<Inst>) -> usize {
        let mut stuck = 0;
        let mut a = CommunicatingDuet::new(0, instrs.clone());
        let mut b = CommunicatingDuet::new(1, instrs);
        while stuck < 2 {
            println!("{:?}", b);
            for _ in 0..2 {
                if let Some(send) = a.do_next() {
                    b.buf.push_back(send);
                }
                if let Some(send) = b.do_next() {
                    a.buf.push_back(send);
                }
            }
            stuck = a.stuck() + b.stuck();
        }
        a.send_count
    }
}
