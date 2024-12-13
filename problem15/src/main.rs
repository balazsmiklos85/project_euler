use num_bigint::BigInt;

fn main() {
    let result = binomial(40, 20);
    println!("{}", result);
}

fn binomial(n: u64, k: u64) -> BigInt {
    let n_factorial_div_k_factorial : BigInt = (k + 1..=n)
        .map(|number| BigInt::from(number))
        .product();
    let n_minus_k_factorial : BigInt = (2..=n - k)
        .map(|number| BigInt::from(number))
        .product();
    n_factorial_div_k_factorial / n_minus_k_factorial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(4, 2), BigInt::from(6));
    }
}
