use num::pow;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let result = calculate_smallest_number_divisible_by_all_numbers_up_to(20);
    println!("{}", result);
}

fn calculate_prime_factors(number: u64) -> HashMap<u64, usize> {
    let mut result = HashMap::new();
    let mut to_divide = number;
    let mut possible_factor = 2;
    while to_divide % possible_factor == 0 {
        to_divide /= possible_factor;
        *result.entry(possible_factor).or_insert(0) += 1;
    }
    possible_factor += 1;
    while to_divide >= possible_factor * possible_factor {
        while to_divide % possible_factor == 0 {
            to_divide /= possible_factor;
            *result.entry(possible_factor).or_insert(0) += 1;
        }
        possible_factor += 2;
    }
    if to_divide > 1 {
        *result.entry(to_divide).or_insert(0) += 1;
    }
    result
}

fn calculate_smallest_number_divisible_by_all_numbers_up_to(upper_limit: u64) -> u64 {
    let required_prime_factors = (2..=upper_limit)
        .map(|number| calculate_prime_factors(number))
        .fold(HashMap::new(), |aggregated, next_factors| {
            aggregate_max_values(aggregated, next_factors)
        });
    required_prime_factors
        .iter()
        .map(|(factor, count)| pow(*factor, *count))
        .product()
}

fn aggregate_max_values(
    mut fold_into: HashMap<u64, usize>,
    to_fold: HashMap<u64, usize>,
) -> HashMap<u64, usize> {
    for (key, value) in to_fold {
        fold_into
            .entry(key)
            .and_modify(|current_value: &mut usize| {
                *current_value = cmp::max(*current_value, value)
            })
            .or_insert(value);
    }
    fold_into
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_smallest_number_divisible_by_all_numbers_up_to_10() {
        assert_eq!(
            calculate_smallest_number_divisible_by_all_numbers_up_to(10),
            2520
        );
    }
}
