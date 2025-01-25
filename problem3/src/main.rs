fn main() {
    println!("{}", calculate_largest_prime_factor(600851475143));
}

/// Calculates the largest prime factor of a number.
///
/// # Arguments
///
/// * `number` - The number to calculate the largest prime factor of.
///
/// # Returns
///
/// The largest prime factor of the number, possibly including the number itself if it is prime.
fn calculate_largest_prime_factor(number: u64) -> u64 {
    let mut to_divide: u64;
    let mut possible_factor: u64;
    to_divide = divide(number, &2).max(2);
    possible_factor = 3;
    while to_divide >= possible_factor * possible_factor {
        to_divide = divide(to_divide, &possible_factor).max(possible_factor);
        possible_factor += 2;
    }
    to_divide
}

/// Tries to divide a number by a possible factor as many times as possible.
///
/// # Arguments
///
/// * `to_divide` - The number to divide.
/// * `possible_factor` - The possible factor to divide by.
///
/// # Returns
///
/// The number reduced by the divisions.
fn divide(mut to_divide: u64, possible_factor: &u64) -> u64 {
    while to_divide % possible_factor == 0 {
        to_divide /= possible_factor;
    }
    to_divide
}
