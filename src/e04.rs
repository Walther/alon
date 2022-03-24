//! Given a string, your task is to generate all different strings that can be created using its characters.

use std::collections::BTreeSet;

/// Given a string `seed`, generates all unique permutations of the letters. Note: this is most likely `O(n!)`, tested up to `seed.len() == 8`.
pub fn creating_strings(seed: &str) -> Vec<String> {
    let letters: Vec<char> = seed.chars().collect();
    let collection = recurse_helper(letters);

    let results: Vec<String> = collection.into_iter().collect();
    results
}

fn recurse_helper(letters: Vec<char>) -> BTreeSet<String> {
    let mut collection: BTreeSet<String> = BTreeSet::new();

    if !letters.is_empty() {
        // recurse step: add a word for each possible recurse
        for (index, letter) in letters.iter().enumerate() {
            let mut rest = letters.clone();
            // remove the current letter from availability pool
            rest.remove(index);
            // special case: ran out of letters
            if rest.is_empty() {
                let mut last_collection: BTreeSet<String> = BTreeSet::new();
                last_collection.insert(letter.to_string());
                return last_collection;
            }
            // generate all possible child chains
            let chains = recurse_helper(rest);
            for chain in chains {
                // add to words list when finished
                let word: String = letter.to_string() + &chain;
                collection.insert(word);
            }
        }
    }

    // return all words generated
    collection
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn creating_strings_() {
        let result = creating_strings("");
        let expected: Vec<String> = Vec::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn creating_strings_a() {
        let result = creating_strings("a");
        let expected = vec!["a"];
        assert_eq!(result, expected);
    }

    #[test]
    fn creating_strings_ab() {
        let result = creating_strings("ab");
        let expected = vec!["ab", "ba"];
        assert_eq!(result, expected);
    }

    #[test]
    fn creating_strings_aabac() {
        let result = creating_strings("aabac");
        let expected = vec![
            "aaabc", "aaacb", "aabac", "aabca", "aacab", "aacba", "abaac", "abaca", "abcaa",
            "acaab", "acaba", "acbaa", "baaac", "baaca", "bacaa", "bcaaa", "caaab", "caaba",
            "cabaa", "cbaaa",
        ];
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_creating_strings_abcdefgj(b: &mut Bencher) {
        b.iter(|| creating_strings("abcdefgj"));
    }
}

// Hack: include a `main` function in the file. This way this file is possible to return as a standalone solution.
#[allow(dead_code)]
fn main() {
    use std::io::{BufRead, BufReader};

    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let seed: &str = line.trim();
    let results = creating_strings(seed);
    println!("{}", results.len());
    for result in results {
        println!("{}", result);
    }
}
