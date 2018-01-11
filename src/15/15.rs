// --- Day 15: Dueling Generators ---
// Here, you encounter a pair of dueling generators. The generators, called generator A and generator B, are trying to agree on a sequence of numbers. However, one of them is malfunctioning, and so the sequences don't always match.

// As they do this, a judge waits for each of them to generate its next value, compares the lowest 16 bits of both values, and keeps track of the number of times those parts of the values match.

// The generators both work on the same principle. To create its next value, a generator will take the previous value it produced, multiply it by a factor (generator A uses 16807; generator B uses 48271), and then keep the remainder of dividing that resulting product by 2147483647. That final remainder is the value it produces next.

struct Generator {
    factor: u64,
    prev: u64,
}

impl Generator {
    fn new(seed: u64, factor: u64) -> Self {
        Generator { prev: seed, factor }
    }
    fn new_a(seed: u64) -> Self {
       Generator::new(seed, 16807)
    }
    fn new_b(seed: u64) -> Self {
       Generator::new(seed, 48271)
    }
}
impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = (self.prev * self.factor) % 2147483647;
        self.prev = val;
        Some(val)
    }
}
// To calculate each generator's first value, it instead uses a specific starting value as its "previous value" (as listed in your puzzle input).

// For example, suppose that for starting values, generator A uses 65, while generator B uses 8921. Then, the first five pairs of generated values are:

// --Gen. A--  --Gen. B--
//    1092455   430625591
// 1181022009  1233683848
//  245556042  1431495498
// 1744312007   137874439
// 1352636452   285222916

#[cfg(test)]
mod tests {
    use Generator;
    #[test]
    fn a_first_five() {
        let a = Generator::new_a(65);
        let first_5 = a.take(5).collect::<Vec<u64>>();
        assert_eq!(first_5, vec![1092455, 1181022009, 245556042, 1744312007, 1352636452])
    }

    #[test]
    fn b_first_five() {
        let b = Generator::new_b(8921);
        let first_5 = b.take(5).collect::<Vec<u64>>();
        assert_eq!(first_5, vec![430625591, 1233683848, 1431495498, 137874439, 285222916])
    }
}
// In binary, these pairs are (with generator A's value first in each pair):

// 00000000000100001010101101100111
// 00011001101010101101001100110111

// 01000110011001001111011100111001
// 01001001100010001000010110001000

// 00001110101000101110001101001010
// 01010101010100101110001101001010

// 01100111111110000001011011000111
// 00001000001101111100110000000111

// 01010000100111111001100000100100
// 00010001000000000010100000000100

// Here, you can see that the lowest (here, rightmost) 16 bits of the third value match: 1110001101001010. Because of this one match, after processing these five pairs, the judge would have added only 1 to its total.

fn match_16(a: &u64, b: &u64) -> bool {
    *a & 65535 == *b & 65535
}

#[cfg(test)]
mod test_bin_match {
    use Generator;
    use match_16;
    #[test]
    fn first_five_matches() {
        let a = Generator::new_a(65);
        let b = Generator::new_b(8921);

        let matches = a
            .take(5)
            .zip(b)
            .map(|(a, b)| match_16(&a, &b))
            .collect::<Vec<bool>>();
        assert_eq!(matches, [false, false, true, false, false]);
    }
}
// To get a significant sample, the judge would like to consider 40 million pairs. (In the example above, the judge would eventually find a total of 588 pairs that match in their lowest 16 bits.)

// After 40 million pairs, what is the judge's final count?

// Input:
// Generator A starts with 699
// Generator B starts with 124

fn part_one() {
    let a = Generator::new_a(699);
    let b = Generator::new_b(124);
    let sum : u64 = a
        .take(40_000_000)
        .zip(b)
        .map(|(a, b)| match_16(&a, &b))
        .map(|b| if b { 1 } else { 0 })
        .sum();
    println!("Part 1: {} matches", sum);
}
// --- Part Two ---
// In the interest of trying to align a little better, the generators get more picky about the numbers they actually give to the judge.

// They still generate values in the same way, but now they only hand a value to the judge when it meets their criteria:

// Generator A looks for values that are multiples of 4.
// Generator B looks for values that are multiples of 8.
// Each generator functions completely independently: they both go through values entirely on their own, only occasionally handing an acceptable value to the judge, and otherwise working through the same sequence of values as before until they find one.

// The judge still waits for each generator to provide it with a value before comparing them (using the same comparison method as before). It keeps track of the order it receives values; the first values from each generator are compared, then the second values from each generator, then the third values, and so on.

// Using the example starting values given above, the generators now produce the following first five values each:

// --Gen. A--  --Gen. B--
// 1352636452  1233683848
// 1992081072   862516352
//  530830436  1159784568
// 1980017072  1616057672
//  740335192   412269392
// These values have the following corresponding binary values:
#[cfg(test)]
mod picky_gnerators {
    use Generator;

    fn a_first_five() {
        let a = Generator::new_a(65).filter(|v| v % 4 == 0);
        let first_5 = a.take(5).collect::<Vec<u64>>();
        assert_eq!(first_5, vec![1352636452, 1992081072, 530830436, 1980017072, 740335192])
    }

    #[test]
    fn b_first_five() {
        let b = Generator::new_b(8921).filter(|v| v % 8 == 0);
        let first_5 = b.take(5).collect::<Vec<u64>>();
        assert_eq!(first_5, vec![1233683848, 862516352, 1159784568, 1616057672, 412269392])
    }
}
// 01010000100111111001100000100100
// 01001001100010001000010110001000

// 01110110101111001011111010110000
// 00110011011010001111010010000000

// 00011111101000111101010001100100
// 01000101001000001110100001111000

// 01110110000001001010100110110000
// 01100000010100110001010101001000

// 00101100001000001001111001011000
// 00011000100100101011101101010000
// Unfortunately, even though this change makes more bits similar on average, none of these values' lowest 16 bits match. Now, it's not until the 1056th pair that the judge finds the first match:

#[cfg(test)] 
mod picky_matches {
    use Generator;
    use match_16;

    #[test]
    fn first_five_matches() {
        let a = Generator::new_a(65).filter(|v| v % 4 == 0);
        let b = Generator::new_b(8921).filter(|v| v % 8 == 0);

        let matches = a
            .zip(b)
            .map(|(a, b)| match_16(&a, &b));
        let mut count = 0;
        for m in matches {
            count += 1;
            if m {
                break;
            }
        }
        assert_eq!(count, 1056);
    }
}
// --Gen. A--  --Gen. B--
// 1023762912   896885216

// 00111101000001010110000111100000
// 00110101011101010110000111100000
// This change makes the generators much slower, and the judge is getting impatient; it is now only willing to consider 5 million pairs. (Using the values from the example above, after five million pairs, the judge would eventually find a total of 309 pairs that match in their lowest 16 bits.)

#[cfg(test)] 
mod picky_match_count {
    use Generator;
    use match_16;

    #[test]
    fn picky_count() {
        let a = Generator::new_a(65).filter(|v| v % 4 == 0);
        let b = Generator::new_b(8921).filter(|v| v % 8 == 0);

        let matches : u64 = a
            .take(5_000_000)
            .zip(b)
            .map(|(a, b)| match_16(&a, &b))
            .map(|v| if v { 1 } else { 0 })
            .sum();

        assert_eq!(matches, 309);
    }
}
// After 5 million pairs, but using this new generator logic, what is the judge's final count?
fn part_two() {
     let a = Generator::new_a(699).filter(|v| v % 4 == 0);
    let b = Generator::new_b(124).filter(|v| v % 8 == 0);

    let matches : u64 = a
        .take(5_000_000)
        .zip(b)
        .map(|(a, b)| match_16(&a, &b))
        .map(|v| if v { 1 } else { 0 })
        .sum();
    
    println!("Part 2: {} matches", matches);
}

fn main() {
    part_one();
    part_two();
}