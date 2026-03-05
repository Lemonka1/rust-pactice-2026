#![allow(non_snake_case, dead_code, unused_imports, unused_variables)]

use std::io::{self, BufRead};

pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();
    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_meet() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_no_meet_slower() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_no_meet_remainder() {
        assert_eq!(kangaroo(43, 2, 70, 2), "NO");
    }
}