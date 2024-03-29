//! Consider an algorithm that takes as input a positive integer n. If n is even, the algorithm divides it by two, and if n is odd, the algorithm multiplies it by three and adds one. The algorithm repeats this, until n is one.

#![feature(test)]
extern crate test;

/// Given a positive integer, returns a string with the Collatz sequence starting from that number, space-separated.
///
/// # Examples
/// ```rust
/// # use alon::*;
/// let result = weird_algorithm(3);
/// let expected = "3 10 5 16 8 4 2 1";
/// assert_eq!(result, expected);
/// ```
pub fn weird_algorithm(start: u64) -> String {
    if start == 0 {
        panic!("Expected positive integer.")
    }

    if start == 1 {
        return "1".to_string();
    }

    match start % 2 {
        0 => format!("{} {}", start, &weird_algorithm(start / 2)),
        1 => format!("{} {}", start, &weird_algorithm(start * 3 + 1)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    #[should_panic]
    fn start_0() {
        let _result = weird_algorithm(0);
    }

    #[test]
    fn start_1() {
        let result = weird_algorithm(1);
        let expected = "1";
        assert_eq!(result, expected);
    }

    #[test]
    fn start_2() {
        let result = weird_algorithm(2);
        let expected = "2 1";
        assert_eq!(result, expected);
    }

    #[test]
    fn start_3() {
        let result = weird_algorithm(3);
        let expected = "3 10 5 16 8 4 2 1";
        assert_eq!(result, expected);
    }

    #[test]
    fn start_10e6() {
        let result = weird_algorithm(1_000_000);
        let expected = "1000000 500000 250000 125000 62500 31250 15625 46876 23438 11719 35158 17579 52738 26369 79108 39554 19777 59332 29666 14833 44500 22250 11125 33376 16688 8344 4172 2086 1043 3130 1565 4696 2348 1174 587 1762 881 2644 1322 661 1984 992 496 248 124 62 31 94 47 142 71 214 107 322 161 484 242 121 364 182 91 274 137 412 206 103 310 155 466 233 700 350 175 526 263 790 395 1186 593 1780 890 445 1336 668 334 167 502 251 754 377 1132 566 283 850 425 1276 638 319 958 479 1438 719 2158 1079 3238 1619 4858 2429 7288 3644 1822 911 2734 1367 4102 2051 6154 3077 9232 4616 2308 1154 577 1732 866 433 1300 650 325 976 488 244 122 61 184 92 46 23 70 35 106 53 160 80 40 20 10 5 16 8 4 2 1";
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_start_10e6(b: &mut Bencher) {
        b.iter(|| weird_algorithm(1_000_000));
    }
}

// Hack: include a `main` function in the file. This way this file is possible to return as a standalone solution.
#[allow(dead_code)]
fn main() {
    use std::io::{BufRead, BufReader};

    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let start: u64 = line.trim().parse().unwrap();
    println!("{}", weird_algorithm(start));
}
