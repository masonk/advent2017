// --- Day 17: Spinlock ---
// Suddenly, whirling in the distance, you notice what looks like a massive, pixelated hurricane: a deadly spinlock. This spinlock isn't just consuming computing power, but memory, too; vast, digital mountains are being ripped from the ground and consumed by the vortex.

// If you don't move quickly, fixing that printer will be the least of your problems.

// This spinlock's algorithm is simple but efficient, quickly consuming everything in its path. It starts with a circular buffer containing only the value 0, which it marks as the current position. It then steps forward through the circular buffer some number of steps (your puzzle input) before inserting the first new value, 1, after the value it stopped on. The inserted value becomes the current position. Then, it steps forward from there the same number of steps, and wherever it stops, inserts after it the second new value, 2, and uses that as the new current position again.

// It repeats this process of stepping forward, inserting a new value, and using the location of the inserted value as the new current position a total of 2017 times, inserting 2017 as its final operation, and ending with a total of 2018 values (including 0) in the circular buffer.

// For example, if the spinlock were to step 3 times per insert, the circular buffer would begin to evolve like this (using parentheses to mark the current position after each iteration of the algorithm):

// (0), the initial state before any insertions.
// 0 (1): the spinlock steps forward three times (0, 0, 0), and then inserts the first value, 1, after it. 1 becomes the current position.
// 0 (2) 1: the spinlock steps forward three times (0, 1, 0), and then inserts the second value, 2, after it. 2 becomes the current position.
// 0  2 (3) 1: the spinlock steps forward three times (1, 0, 2), and then inserts the third value, 3, after it. 3 becomes the current position.
// And so on:

// 0  2 (4) 3  1
// 0 (5) 2  4  3  1
// 0  5  2  4  3 (6) 1
// 0  5 (7) 2  4  3  6  1
// 0  5  7  2  4  3 (8) 6  1
// 0 (9) 5  7  2  4  3  8  6  1
// Eventually, after 2017 insertions, the section of the circular buffer near the last insertion looks like this:

// 1512  1134  151 (2017) 638  1513  851
// Perhaps, if you can identify the value that will ultimately be after the last value written (2017), you can short-circuit the spinlock. In this example, that would be 638.

// What is the value after 2017 in your completed circular buffer?

// Your puzzle input is 328.

#[derive(Debug)]
struct Spinlock {
    cursor: usize,
    buf: Vec<u32>,
    steplen: usize,
}

impl Spinlock {
    fn new(steplen: usize, steps: &u32) -> Self {
        let mut buf: Vec<u32> = Vec::with_capacity(*steps as usize);
        buf.push(0);
        let cursor = 0;
        let mut spinlock = Spinlock {
            buf,
            cursor,
            steplen,
        };
        spinlock.expand(&steps);
        spinlock
    }

    fn expand(&mut self, steps: &u32) {
        for i in 1..(*steps + 1) {
            self.cursor = 1 + ((self.cursor + self.steplen) % i as usize);
            self.buf.insert(self.cursor, i);
        }
    }

    fn to_string(&self) -> String {
        self.buf
            .iter()
            .enumerate()
            .map(|(i, val)| {
                if i == self.cursor {
                    return format!("({})", val);
                }
                return format!("{}", val);
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn after(&self) -> u32 {
        self.buf[(self.cursor + 1) % self.buf.len()]
    }

    fn after_zero(&self) -> u32 {
        self.buf[1]
    }
}

#[cfg(test)]
mod test_spinlock {
    use Spinlock;
    #[test]
    fn first_four_example_states() {
        let spinlock = Spinlock::new(3, &3);
        assert_eq!(&spinlock.to_string(), "0 2 (3) 1");
    }
    #[test]
    fn example_final_state() {
        let spinlock = Spinlock::new(3, &2017);
        let after = spinlock.after();
        assert_eq!(after, 638);
    }
}

fn part_one() -> u32 {
    let spinlock = Spinlock::new(328, &2017);
    spinlock.after()
}

fn part_two() -> u32 {
    let mut after_zero = 0;
    let mut cursor = 0;
    for i in 1..50_000_000 {
        cursor = 1 + ((cursor + 328) % i);
        if cursor == 1 {
            after_zero = i;
        }
    }
    after_zero
}
fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
