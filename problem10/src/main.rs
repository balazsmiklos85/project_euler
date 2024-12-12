fn main() {
    let result = sum_of_primes_below(2_000_000);
    println!("{}", result);
}

fn is_prime(number: u64, primes: &Vec<u64>) -> bool {
    primes.iter().all(|prime| number % prime != 0)
}

fn sum_of_primes_below(upper_limit: u64) -> u64 {
    let mut primes = Vec::new();
    for number in std::iter::once(2).chain((3..upper_limit).step_by(2)) {
        if is_prime(number, &primes) {
            primes.push(number);
            if primes.len() % 1000 == 0 {
                println!("Found prime: {}", number);
            }
        }
    }
    println!("{:?}", primes);
    primes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_primes_below_10() {
        assert_eq!(sum_of_primes_below(10), 17);
    }
}
