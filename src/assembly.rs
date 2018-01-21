// --- Day 18: Duet ---
// You discover a tablet containing some strange assembly code labeled simply "Duet". Rather than bother the sound card with it, you decide to run the code yourself. Unfortunately, you don't see any documentation, so you're left to figure out what the instructions mean on your own.

// It seems like the assembly is meant to operate on a set of registers that are each named with a single letter and that can each hold a single integer. You suppose each register should start with a value of 0.

// There aren't that many instructions, so it shouldn't be hard to figure out what they do. Here's what you determine:

// snd X plays a sound with a frequency equal to the value of X.
// set X Y sets register X to the value of Y.
// add X Y increases register X by the value of Y.
// mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
// mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
// rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
// jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
// Many of the instructions can take either a register (a single letter) or a number. The value of a register is the integer it contains; the value of a number is that number.

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
    Rcv(Val),
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
                if let &Rcv(ref v) = inst {
                    if self.resolve(v) != 0 {
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
// After each jump instruction, the program continues with the instruction to which the jump jumped. After any other instruction, the program continues with the next instruction. Continuing (or jumping) off either end of the program terminates it.

// What is the value of the recovered frequency (the value of the most recently played sound) the first time a rcv instruction is executed with a non-zero value?
