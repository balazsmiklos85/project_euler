use std::collections::HashSet;

fn main() {
    let abundants = abundant_numbers_up_to(28123);
    let abundant_sums = abundant_sums(abundants, 28123);

    let sum_of_numbers = 395_465_626; // 28123 * (28123 + 1) / 2
    let sum_of_abundant_sums = abundant_sums.iter().sum::<u64>();
    let sum_of_non_abundant_numbers = sum_of_numbers - sum_of_abundant_sums;

    println!("{}", sum_of_non_abundant_numbers);
}

fn abundant_numbers_up_to(limit: u64) -> Vec<u64> {
    (1..=limit).filter(|n| is_abundant(*n)).collect()
}

fn abundant_sums(abundants: Vec<u64>, limit: u64) -> HashSet<u64> {
    let mut sums = HashSet::new();
    for &a in &abundants {
        for &b in &abundants {
            if a + b > limit {
                break;
            } else {
                sums.insert(a + b);
            }
        }
    }
    sums
}

fn is_abundant(n: u64) -> bool {
    let divisors = proper_divisors(n);
    let mut sum = 0;
    for divisor in divisors {
        sum += divisor;
        if sum > n {
            return true;
        }
    }
    false
}

fn proper_divisors(n: u64) -> HashSet<u64> {
    (1..n).filter(|d| n % d == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abundant() {
        assert!(is_abundant(12));
        assert!(!is_abundant(28));
    }
}
