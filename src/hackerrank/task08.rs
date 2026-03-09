#![allow(non_snake_case, dead_code, unused_imports, unused_variables)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 */
#[allow(non_snake_case)] // Это уберет warning, но оставит имя как на HackerRank
pub fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let fptr_result = env::var("OUTPUT_PATH");
    let mut fptr = match fptr_result {
        Ok(path) => File::create(path).unwrap(),
        Err(_) => return,
    };

    let n_str = stdin_iterator.next().unwrap().unwrap();
    let _n = n_str.trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_breakingRecords_sample_0() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breakingRecords(&scores), vec![2, 4]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_breakingRecords_sample_1() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breakingRecords(&scores), vec![4, 0]);
    }
}