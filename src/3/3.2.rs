// --- Part Two ---

// As a stress test on the system, the programs here clear the grid and then store the value 1 in square 1. Then, in the same allocation order as shown above, they store the sum of the values in all adjacent squares, including diagonals.

// So, the first few squares' values are chosen as follows:

// Square 1 starts with the value 1.
// Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
// Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
// Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
// Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
// Once a square is written, its value does not change. Therefore, the first few squares would receive the following values:

// 147  142  133  122   59
// 304    5    4    2   57
// 330   10    1    1   54
// 351   11   23   25   26
// 362  747  806--->   ...
// What is the first value written that is larger than your puzzle input?

// Your puzzle input is still 368078.
use std::collections::HashMap;

fn set_number(i : i32, j : i32, values: &mut HashMap<(i32, i32), u32>) -> u32 {
    let mut sum = 0;
    for idx in (i-1)..(i+2) {
        for jdx in (j-1)..(j+2) {
            match values.get(&(idx, jdx)) {
                Some(val) => { sum += val }
                None => {}
            }
        }
    }
    values.insert((i, j), sum);
    return sum;
}

fn main() {
    let input = 368078;
    
    let mut values = HashMap::new();
    values.insert((0, 0), 1);

    let mut i = 0;
    let mut j = 0;
    let mut layer = 0;

    'outer: loop {
        layer += 1;
        let sidelen = layer * 2;

        i += 1;
        let num = set_number(i, j, &mut values);
        println!("{}", num);
        if num > input {
            break 'outer;
        }
        
        for _ in 0..(sidelen - 1) {
            j += 1;
            let num = set_number(i, j, &mut values);
            println!("{}", num);
            if num > input {
                break 'outer;
            }
        }
        for _ in 0..sidelen {
            i -= 1;
            let num = set_number(i, j, &mut values);
            println!("{}", num);
            if num > input {
                break 'outer;
            }
        }
        for _ in 0..sidelen {
            j -= 1;
            let num = set_number(i, j, &mut values);
            println!("{}", num);
            if num > input {
                break 'outer;
            }
        }
        for _ in 0..sidelen {
            i += 1;
            let num = set_number(i, j, &mut values);
            println!("{}", num);
            if num > input {
                break 'outer;
            }
        }
        
    }
}

