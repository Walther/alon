//! You are given an array of n integers. You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.
//! On each move, you may increase the value of any element by one. What is the minimum number of moves required?

/// Returns the minimum number of "moves" required in order to make the array monotonically increasing in order, where a "move" is an operation of incrementing any single element by one.
pub fn increasing_array(_n: usize, array: Vec<usize>) -> usize {
    let mut moves = 0;
    let mut iter = array.iter();
    // Start with an accumulator = array[0]
    let mut accumulator = iter.next().unwrap(); // FIXME: remove unwrap

    // Iterate through the rest of the array
    while let Some(number) = iter.next() {
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

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn increasing_32517() {
        let result = increasing_array(5, vec![3, 2, 5, 1, 7]);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
