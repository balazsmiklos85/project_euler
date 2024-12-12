fn main() {
    let result = get_nth_prime(10001);
    match result {
        Some(n) => println!("{}", n),
        None => println!("No prime found"),
    }
}

fn get_nth_prime(n: usize) -> Option<u64> {
    let mut primes: Vec<u64> = Vec::new();
    for number in std::iter::once(2)
        .chain((3..).step_by(2)) {
            if is_prime(number, &primes) {
                primes.push(number);
            }
            if primes.len() == n {
                return primes.last().copied();
            }
        }
    None
}

fn is_prime(number: u64, primes: &Vec<u64>) -> bool {
    primes.iter().all(|&prime| number % prime != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_prime() {
        assert_eq!(get_nth_prime(6), Some(13));
    }
}
