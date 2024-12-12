use prime_number_utils::baillie_psw;

fn main() {
    let result = sum_of_primes_below(2_000_000);
    println!("{}", result);
}

fn sum_of_primes_below(upper_limit: usize) -> u64 {
    let mut sum : u64 = 2;
    for number in (3..upper_limit).step_by(2) {
        if baillie_psw(number) {
            sum += number as u64;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_primes_below_10() {
        assert_eq!(sum_of_primes_below(10), 17);
    }
}
