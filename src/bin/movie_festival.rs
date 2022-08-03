//! In a movie festival n movies will be shown. You know the starting and ending time of each movie. What is the maximum number of movies you can watch entirely?

#![feature(test)]
extern crate test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Movie {
    start: u32,
    end: u32,
}

impl From<(u32, u32)> for Movie {
    fn from((start, end): (u32, u32)) -> Self {
        Movie { start, end }
    }
}

/// Given a Vec of `Movie`s, return the maximum possible count of non-overlapping movies
pub fn movie_festival(mut movies: Vec<Movie>) -> u32 {
    let mut movie_count = 0;
    let mut current_time = 0;

    // Sort by end times
    movies.sort_by(|a, b| a.end.cmp(&b.end));

    // Go through all the sorted movies
    for movie in movies {
        // Watch the movie if it hasn't started yet
        if movie.start >= current_time {
            movie_count += 1;
            current_time = movie.end;
        }
    }

    movie_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movie_festival_3() {
        let input = vec![(3, 5), (4, 9), (5, 8)]
            .iter()
            .map(|&m| m.into())
            .collect();
        let result = movie_festival(input);
        let expected: u32 = 2;
        assert_eq!(result, expected);
    }
}

// Hack: include a `main` function in the file. This way this file is possible to return as a standalone solution.
#[allow(dead_code)]
fn main() {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut movies = Vec::new();
    // Number of lines
    stdin.read_line(&mut buffer).unwrap();
    let count: u32 = buffer.trim().parse().unwrap();

    for _ in 0..count {
        buffer = "".into();
        stdin.read_line(&mut buffer).unwrap();
        let split: Vec<&str> = buffer.trim().split(' ').collect();
        let start = split[0].parse().unwrap();
        let end = split[1].parse().unwrap();
        movies.push(Movie { start, end });
    }
    let count = movie_festival(movies);
    println!("{}", count);
}
