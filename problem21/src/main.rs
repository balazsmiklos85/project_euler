fn main() {
    let result : u64 = amicable_numbers().take_while(|number| number < &10_000).sum();
    println!("{}", result);
}

fn amicable_numbers() -> impl Iterator<Item = u64> {
    (1..).filter(|number| amicable_pair(*number))
}

fn amicable_pair(number: u64) -> bool {
    let possible_pair = sum_of_divisors(number);
    number != possible_pair && sum_of_divisors(possible_pair) == number
}

fn sum_of_divisors(number: u64) -> u64 {
    (1..number).filter(|divisor| number % divisor == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_divisors() {
        assert_eq!(sum_of_divisors(220), 284);
        assert_eq!(sum_of_divisors(284), 220);
    }
}
