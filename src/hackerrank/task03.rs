use std::io::{self, BufRead};

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            staircase(n);
        }
    }
}