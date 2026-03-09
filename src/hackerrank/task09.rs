#![allow(dead_code, unused_imports, non_snake_case)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 */

#[allow(non_snake_case)]
pub fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut iter = scores.iter();

    let first = match iter.next() {
        Some(&val) => val,
        None => return vec![0, 0],
    };

    let (mut max_s, mut min_s) = (first, first);
    let (mut max_c, mut min_c) = (0, 0);

    for &score in iter {
        if score > max_s {
            max_s = score;
            max_c += 1;
        } else if score < min_s {
            min_s = score;
            min_c += 1;
        }
    }

    vec![max_c, min_c]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

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
    fn test_breakingRecords_case_0() {
        let input = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breakingRecords(&input), vec![2, 4]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_breakingRecords_case_1() {
        let input = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breakingRecords(&input), vec![4, 0]);
    }
}