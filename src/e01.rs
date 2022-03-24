//! You are given an array of n integers. You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.
//! On each move, you may increase the value of any element by one. What is the minimum number of moves required?

/// Returns the minimum number of "moves" required in order to make the array monotonically increasing in order, where a "move" is an operation of incrementing any single element by one.
pub fn increasing_array(array: Vec<u64>) -> u64 {
    let mut moves = 0;
    let mut iter = array.iter();
    // Start with an accumulator = array[0]
    let mut accumulator = iter.next().unwrap(); // FIXME: remove unwrap

    // Iterate through the rest of the array
    for number in iter {
        // If the number is equal to the accumulator, continue
        if number == accumulator {
            continue;
        }
        // If the number is larger than the current accumulator, make accumulator = number and continue
        if number > accumulator {
            accumulator = number;
            continue;
        }
        // If the number is smaller than the current accumulator, increment until it is at least as large as the accumulator. Keep the accumulator, as they're now equal.
        moves += accumulator - number;
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn increasing_32517() {
        let result = increasing_array(vec![3, 2, 5, 1, 7]);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn increasing_1() {
        let result = increasing_array(vec![1]);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn increasing_123() {
        let result = increasing_array(vec![1, 2, 3]);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_increasing_32517(b: &mut Bencher) {
        // TODO: better benchmark
        b.iter(|| increasing_array(vec![3, 2, 5, 1, 7]));
    }
}

// Hack: include a `main` function in the file. This way this file is possible to return as a standalone solution.
#[allow(dead_code)]
fn main() {
    use std::io::{BufRead, BufReader};

    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let _n: u64 = line.trim().parse().unwrap();

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let split = line.split_whitespace();
    let array: Vec<u64> = split.map(|i| i.parse().unwrap()).collect();
    println!("{}", increasing_array(array));
}
