use std::collections::HashMap;

fn main() {
    let result = find_triable_number_by_divisor_count(500);
    match result {
        Some(tn) => println!("{}", tn.value),
        None => println!("No triangle number found"),
    }
}

struct TriangleNumber {
    value: u64,
    factors: usize,
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

fn count_factors(prime_factors: HashMap<u64, usize>) -> usize {
    prime_factors
        .into_iter()
        .map(|(_, count)| count + 1)
        .product()
}

fn find_triable_number_by_divisor_count(divisor_count: usize) -> Option<TriangleNumber> {
    triangle_numbers().find(|tn| tn.factors >= divisor_count)
}

fn triangle_numbers() -> impl Iterator<Item = TriangleNumber> {
    (1..).map(|i| {
        let value = i * (i + 1) / 2;
        let factors = calculate_prime_factors(value);
        TriangleNumber { value, factors: count_factors(factors) }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_triable_number_by_divisor_count() {
        let result = find_triable_number_by_divisor_count(5).unwrap();
        assert_eq!(result.value, 28);
        assert_eq!(result.factors, 6);
    }
}
