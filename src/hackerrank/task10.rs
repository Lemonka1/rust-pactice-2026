#![allow(dead_code, unused_imports, non_snake_case)]

use std::collections::HashSet;

/**
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 * 1. INTEGER n
 * 2. INTEGER_ARRAY ar
 */
pub fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut set = HashSet::new();
    let mut pairs = 0;

    for i in 0..n as usize {
        if !set.insert(ar[i]) {
            set.remove(&ar[i]);
            pairs += 1;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let n = 9;
        let ar = [10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sockMerchant(n, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_example2() {
        let n = 7;
        let ar = [1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sockMerchant(n, &ar), 2);
    }
}