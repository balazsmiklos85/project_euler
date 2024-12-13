use num_bigint::BigInt;

fn main() {
    let result = sum_power_of_two_digits(1000);
    println!("{}", result);
}

fn sum_power_of_two_digits(exponent: u32) -> u64 {
    BigInt::from(2)
        .pow(exponent)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_power_of_two_digits() {
        assert_eq!(sum_power_of_two_digits(15), 26);
    }
}
