use num_bigint::BigInt;

fn main() {
    let factorial = factorial(100);
    let result = sum_digits(factorial);
    println!("{}", result);
}

fn factorial(n: u64) -> BigInt {
    (2..=n).map(BigInt::from).product()
}

fn sum_digits(n: BigInt) -> u64 {
    n.to_string().chars().map(|c| c.to_digit(10).map(|d| d as u64).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), BigInt::from(3628800));
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(BigInt::from(3628800)), 27);
    }

}
