fn main() {
    println!("{}", calculate_largest_prime_factor(600851475143).unwrap());
}

fn calculate_largest_prime_factor(number: u64) -> Result<u64, String> {
    let mut largest_factor = Err("The number is a prime.".to_string());
    let mut to_divide = number;
    let mut possible_factor = 2;
    while to_divide % possible_factor == 0 {
        largest_factor = Ok(possible_factor);
        to_divide /= possible_factor;
    }
    possible_factor += 1;
    while possible_factor * possible_factor <= to_divide {
        while to_divide % possible_factor == 0 {
            largest_factor = Ok(possible_factor);
            to_divide /= possible_factor;
        }
        possible_factor += 2;
    }
    largest_factor
}

