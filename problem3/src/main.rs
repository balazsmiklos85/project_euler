fn main() {
    let result = calculate_largest_prime_factor(600851475143);
    match result {
        Ok(factor) => println!("{}", factor),
        Err(error) => println!("{}", error),
    }
}

fn calculate_largest_prime_factor(number: u64) -> Result<u64, String> {
    let mut to_divide: u64;
    let mut possible_factor: u64;
    (to_divide, possible_factor) = divide(number, 2, 1);
    while to_divide > possible_factor * possible_factor {
        (to_divide, possible_factor) = divide(to_divide, possible_factor, 2);
    }
    if to_divide == number {
        Err("The number is a prime.".to_string())
    } else {
        Ok(to_divide)
    }
}

fn divide(to_divide: u64, possible_factor: u64, step: u64) -> (u64, u64) {
    let mut to_divide = to_divide;
    let mut possible_factor = possible_factor;
    while to_divide % possible_factor == 0 {
        to_divide /= possible_factor;
    }
    possible_factor += step;
    (to_divide, possible_factor)
}
