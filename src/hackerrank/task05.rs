#![allow(non_snake_case, dead_code, unused_imports, unused_variables)]

use std::io::{self, BufRead};

pub fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter()
        .filter(|&&d| a + d >= s && a + d <= t)
        .count();

    let orange_count = oranges.iter()
        .filter(|&&d| b + d >= s && b + d <= t)
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _m = third_multiple_input[0].trim().parse::<i32>().unwrap();
    let _n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fruit_counts(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (usize, usize) {
        let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
        let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();
        (apple_count, orange_count)
    }

    #[test]
    fn test_sample_case() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let (apple_count, orange_count) = get_fruit_counts(s, t, a, b, &apples, &oranges);
        assert_eq!(apple_count, 1);
        assert_eq!(orange_count, 1);
    }
}