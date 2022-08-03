//! You are given an array that contains each number between 1…n exactly once. Your task is to collect the numbers from 1 to n in increasing order.
//! On each round, you go through the array from left to right and collect as many numbers as possible. What will be the total number of rounds?

use std::collections::HashMap;

/// Returns the number of rounds required to collect every number 1..n in increasing order, when reading the array from left to right, always only picking numbers in order.
pub fn collecting_numbers(numbers: &[usize]) -> usize {
    let max = numbers.len();

    // Create a number-index lookup table
    let mut indices: HashMap<usize, usize> = HashMap::new();
    for (index, &number) in numbers.iter().enumerate() {
        indices.insert(number, index);
    }

    // First iteration starts from the beginning
    let mut current_index = 0;
    let mut iterations = 1;

    // For each target number, check its index. If we go backwards, increment iteration count
    for target in 1..=max {
        let target_index = indices.get(&target).unwrap();
        if target_index < &current_index {
            iterations += 1;
        }
        current_index = *target_index;
    }

    iterations
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn collecting_numbers_1() {
        let result = collecting_numbers(&[1]);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn collecting_numbers_12() {
        let result = collecting_numbers(&[1, 2]);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn collecting_numbers_21() {
        let result = collecting_numbers(&[2, 1]);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn collecting_numbers_42153() {
        let result = collecting_numbers(&[4, 2, 1, 5, 3]);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_collecting_numbers_1_2(b: &mut Bencher) {
        let v = (1..2).rev().collect::<Vec<usize>>();
        println!("{:?}", &v);
        b.iter(|| collecting_numbers(&v));
    }

    #[bench]
    fn bench_collecting_numbers_1_10(b: &mut Bencher) {
        let v = (1..10).rev().collect::<Vec<usize>>();
        println!("{:?}", &v);
        b.iter(|| collecting_numbers(&v));
    }

    #[bench]
    fn bench_collecting_numbers_1_100(b: &mut Bencher) {
        let v = (1..100).rev().collect::<Vec<usize>>();
        println!("{:?}", &v);
        b.iter(|| collecting_numbers(&v));
    }

    #[bench]
    fn bench_collecting_numbers_1_1000(b: &mut Bencher) {
        let v = (1..1000).rev().collect::<Vec<usize>>();
        println!("{:?}", &v);
        b.iter(|| collecting_numbers(&v));
    }
}

// Hack: include a `main` function in the file. This way this file is possible to return as a standalone solution.
#[allow(dead_code)]
fn main() {
    use std::io::{BufRead, BufReader};

    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let _n: usize = line.trim().parse().unwrap();

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let split = line.split_whitespace();
    let array: Vec<usize> = split.map(|i| i.parse().unwrap()).collect();
    println!("{}", collecting_numbers(&array));
}
