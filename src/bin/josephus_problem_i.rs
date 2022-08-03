//! Consider a game where there are n children (numbered 1,2,…,n) in a circle. During the game, every second child is removed from the circle, until there are no children left. In which order will the children be removed?

use std::collections::VecDeque;

/// Given a number of children to play, returns the order in which the children get removed from the game.
pub fn josephus_problem_i(n: u64) -> Vec<u64> {
    let mut circle: VecDeque<u64> = (1..=n).collect();
    let mut order: Vec<u64> = Vec::new();

    while !circle.is_empty() {
        circle.rotate_left(1);
        order.push(circle.pop_front().unwrap());
    }

    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn josephus_problem_i_7() {
        let input = 7;
        let result = josephus_problem_i(input);
        let expected = vec![2, 4, 6, 1, 5, 3, 7];
        assert_eq!(result, expected);
    }
}

fn main() {
    use std::io::{BufRead, BufReader};

    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let start: u64 = line.trim().parse().unwrap();
    println!(
        "{}",
        josephus_problem_i(start)
            .iter()
            .map(|&n| format!("{}", n))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
