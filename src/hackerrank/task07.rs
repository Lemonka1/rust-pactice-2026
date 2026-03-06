#![allow(non_snake_case, dead_code, unused_imports, unused_variables)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'getTotalX' function below.
 */
pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    // Нахождение наибольшего общего делителя (НОД)
    let gcd = |mut x: i32, mut y: i32| {
        while y != 0 {
            x %= y;
            std::mem::swap(&mut x, &mut y);
        }
        x
    };

    // Нахождение наименьшего общего кратного (НОК)
    let lcm = |x: i32, y: i32| {
        if x == 0 || y == 0 { 0 } else { (x * y).abs() / gcd(x, y) }
    };

    // Вычисляем НОК для всего массива 'a'
    let lcm_a = a.iter().fold(a[0], |acc, &x| lcm(acc, x));

    // Вычисляем НОД для всего массива 'b'
    let gcd_b = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    // Считаем количество чисел кратных lcm_a, которые являются делителями gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // В HackerRank путь к файлу берется из переменной окружения
    let fptr_result = env::var("OUTPUT_PATH");
    let mut fptr = match fptr_result {
        Ok(path) => File::create(path).unwrap(),
        Err(_) => return, // Для локального запуска без переменной окружения
    };

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let _m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getTotalX_sample() {
        // Тест из Sample Input: a=[2, 4], b=[16, 32, 96] -> Ответ: 3
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(getTotalX(&a, &b), 3);
    }

    #[test]
    fn test_getTotalX_example() {
        // Тест из Example: a=[2, 6], b=[24, 36] -> Ответ: 2
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(getTotalX(&a, &b), 2);
    }
}