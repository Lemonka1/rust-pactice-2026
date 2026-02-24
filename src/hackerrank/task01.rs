#![allow(non_snake_case)]

pub fn simpleArraySum(ar: &[i32]) -> i32 {
    let mut sum = 0;
    for element in ar {
        sum += element;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpleArraySum() {
        let ar = [1, 2, 3, 4, 10, 11];
        assert_eq!(simpleArraySum(&ar), 31);
    }
}