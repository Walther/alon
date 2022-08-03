//! Your task is to divide the numbers 1,2,…,n into two sets of equal sum.

#![feature(test)]
extern crate test;

/// Given a number `n`, returns the numbers `1..n` split into two sets with equal sum if possible, or `None` otherwise.
pub fn two_sets(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    // Arithmetic series sum
    let sum = (n * (1 + n)) / 2;
    // If the sum is odd, return early
    if sum % 2 != 0 {
        return None;
    }

    // We want both sides to hit this target sum
    let _target = sum / 2;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn n_1() {
        let result = two_sets(1);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn n_2() {
        let result = two_sets(2);
        let expected = None;
        assert_eq!(result, expected);
    }

    /*
    #[test]
    fn n_3() {
        let result = two_sets(3);
        let expected = Some((vec![1, 2], vec![3]));
        assert_eq!(result, expected);
    }

    #[test]
    fn n_4() {
        let result = two_sets(4);
        let expected = Some((vec![1, 4], vec![2, 3]));
        assert_eq!(result, expected);
    }

    #[test]
    fn n_6() {
        let result = two_sets(6);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn n_7() {
        let result = two_sets(7);
        let expected = Some((vec![1, 2, 4, 7], vec![3, 5, 6]));
        assert_eq!(result, expected);
    }


    #[bench]
    fn bench_n_1e6(b: &mut Bencher) {
        b.iter(|| two_sets(1_000_000));
    }

    */
}

fn main() {
    todo!()
}
