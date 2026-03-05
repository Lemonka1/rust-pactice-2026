#![allow(dead_code, unused_imports, non_snake_case)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade >= 38 && grade % 5 >= 3 {
            grade + (5 - (grade % 5))
        } else {
            grade
        }
    }).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students_sample() {
        let grades = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(gradingStudents(&grades), expected);
    }

    #[test]
    fn test_grading_students_no_rounding() {
        let grades = vec![37, 29, 0];
        let expected = vec![37, 29, 0];
        assert_eq!(gradingStudents(&grades), expected);
    }

    #[test]
    fn test_grading_students_exact_multiple() {
        let grades = vec![85, 40, 100];
        let expected = vec![85, 40, 100];
        assert_eq!(gradingStudents(&grades), expected);
    }
}