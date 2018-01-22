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

// After each jump instruction, the program continues with the instruction to which the jump jumped. After any other instruction, the program continues with the next instruction. Continuing (or jumping) off either end of the program terminates it.

// What is the value of the recovered frequency (the value of the most recently played sound) the first time a rcv instruction is executed with a non-zero value?

// --- Part Two ---
// As you congratulate yourself for a job well done, you notice that the documentation has been on the back of the tablet this entire time. While you actually got most of the instructions correct, there are a few key differences. This assembly code isn't about sound at all - it's meant to be run twice at the same time.

// Each running copy of the program has its own set of registers and follows the code independently - in fact, the programs don't even necessarily run at the same speed. To coordinate, they use the send (snd) and receive (rcv) instructions:

// snd X sends the value of X to the other program. These values wait in a queue until that program is ready to receive them. Each program has its own message queue, so a program can never receive a message it sent.
// rcv X receives the next value and stores it in register X. If no values are in the queue, the program waits for a value to be sent to it. Programs do not continue to the next instruction until they have received a value. Values are received in the order they are sent.
// Each program also has its own program ID (one 0 and the other 1); the register p should begin with this value.

// For example:

// snd 1
// snd 2
// snd p
// rcv a
// rcv b
// rcv c
// rcv d
// Both programs begin by sending three values to the other. Program 0 sends 1, 2, 0; program 1 sends 1, 2, 1. Then, each program receives a value (both 1) and stores it in a, receives another value (both 2) and stores it in b, and then each receives the program ID of the other program (program 0 receives 1; program 1 receives 0) and stores it in c. Each program now sees a different value in its own copy of register c.

// Finally, both programs try to rcv a fourth time, but no data is waiting for either of them, and they reach a deadlock. When this happens, both programs terminate.

// It should be noted that it would be equally valid for the programs to run at different speeds; for example, program 0 might have sent all three values and then stopped at the first rcv before program 1 executed even its first instruction.

// Once both of your programs have terminated (regardless of what caused them to do so), how many times did program 1 send a value?

// Although it hasn't changed, you can still get your puzzle input.
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::io::Write;
use std::fs::File;
struct CommunicatingDuet<TLog: Write> {
    tx: Option<Sender<RegisterSize>>, // the channel on which we handle Snd instructions
    rx: Receiver<RegisterSize>,       // Where we wait for Rcv instructions
    waiting_tx: SyncSender<bool>,
    waiting_rx: Receiver<bool>,
    send_count: usize,
    done: bool,
    cursor: usize,
    instrs: Vec<Inst>,
    registers: HashMap<char, RegisterSize>,
    pid: i64,
    log_writer: Option<TLog>,
}
impl<TLog: Write> fmt::Debug for CommunicatingDuet<TLog> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DuetState {{ next_instr: {:?}, registers: {:?}, send_count: {} }}",
            self.instrs[self.cursor], self.registers, self.send_count
        )
    }
}
impl<TLog: Write> CommunicatingDuet<TLog> {
    fn new(
        pid: i64,
        instrs: Vec<Inst>,
        tx: Sender<RegisterSize>,
        rx: Receiver<RegisterSize>,
        waiting_tx: Sender<bool>,
        waiting_rx: Sender<bool>,
        log_writer: Option<TLog>,
    ) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', pid);
        CommunicatingDuet {
            pid,
            tx: Some(tx),
            rx,
            instrs,
            registers,
            send_count: 0,
            cursor: 0,
            done: false,
            waiting_rx,
            waiting_tx,
            log_writer,
        }
    }
    fn log(&mut self, msg: &str) {
        if let Some(w) = self.log_writer {
            w.write(msg);
        }
    }
    pub fn play_to_end(&mut self) {
        loop {
            if self.done {
                break;
            }
            if self.cursor >= self.instrs.len() {
                self.signal_done();
                break;
            }
            self.do_next();
        }
    }
    fn signal_done(&mut self) {
        // Dropping tx causes the receiver half to err when a value is received.
        // This lets the other side knows that no new values are going to coming.
        // We flag done to break our own execution loop so our thread will clean up.
        let tx = self.tx.take();
        drop(tx);
        self.done = true;
    }
    fn do_next(&mut self) {
        let inst = self.instrs[self.cursor].clone();
        let mut jmp = 1;
        self.log(format!(&"{:?}", self));
        match inst {
            Snd(ref v) => {
                self.send_count += 1;
                let v = self.resolve(v);
                if let Some(ref mut tx) = self.tx {
                    tx.send(v).unwrap();
                } else {
                    panic!("Tried to send but the channel was closed.");
                }
            }
            Rcv(a) => {
                match self.rx.try_recv() {
                    Ok(v) => {
                        self.registers.insert(a, v);
                    }
                    Err(e) => {
                        match e {
                            mpsc::TryRecvError::Empty => {
                                // There's nothing on the channel.
                                // Before we block for the next value,
                                // determine if the other side is waiting for
                                // If it is, we know it's a deadlock.

                                match self.rx.recv() {
                                    Ok(v) => {}
                                    Err(_) => {
                                        self.log(&format!(
                                            "Empty buffer, blocking for the next signal. ({:?})",
                                            inst
                                        ));
                                        // No new values will ever come.
                                        // That's a deadlock; by the problem statement, we're done.
                                        self.signal_done();
                                    }
                                }
                            }
                            mpsc::TryRecvError::Disconnected => {
                                // No new values will ever come.
                                // That's a deadlock; by the problem statement, we're done.
                                self.log(&format!(
                                    "Got rcv instruction on a disconnected signal. ({:?})",
                                    inst
                                ));
                                self.signal_done();
                            }
                        }
                    }
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

pub struct DuetPair {
    a: CommunicatingDuet<File>,
    b: CommunicatingDuet<File>,
}
impl DuetPair {
    pub fn play(instrs: Vec<Inst>) -> usize {
        let (from_a, to_b) = mpsc::channel();
        let (from_b, to_a) = mpsc::channel();
        let (wait_from_a, wait_to_b) = mpsc::sync_channel(0);
        let (wait_from_b, wait_to_a) = mpsc::sync_channel(0);

        let a_instrs = instrs.clone();

        let a = thread::spawn(move || {
            let log = File::create("duet_0.log");
            let mut a = CommunicatingDuet::new(
                0,
                a_instrs,
                from_a,
                to_a,
                wait_from_a,
                wait_to_a,
                Some(log),
            );
            a.play_to_end();
            a.send_count
        });
        let b = thread::spawn(move || {
            let log = File::create("duet_1.log");
            let mut b =
                CommunicatingDuet::new(1, instrs, from_b, to_b, wait_from_b, wait_to_b, Some(log));
            b.play_to_end();
            b.send_count
        });

        b.join().unwrap();
        a.join().unwrap()
    }
}
