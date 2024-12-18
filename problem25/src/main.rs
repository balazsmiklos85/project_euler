use num_bigint::BigUint;

fn main() {
    let result = calculate_fibinacci_index_by_length(1000);
    match result {
        Some(index) => println!("{}", index),
        None => println!("There was a problem iterating over the Fibonacci sequence"),
    }
}

fn calculate_fibinacci_index_by_length(length: usize) -> Option<usize> {
    custom_fibonacci_sequence(1, 1)
        .enumerate()
        .filter(|(_index, number)| number.to_string().len() >= length)
        .map(|(index, _)| index + 1)
        .next()
}

fn custom_fibonacci_sequence(first: u64, second: u64) -> impl Iterator<Item = BigUint> {
    let mut first_item = BigUint::from(first);
    let mut second_item = BigUint::from(second);
    std::iter::once(first_item.clone())
        .chain(std::iter::once(second_item.clone()))
        .chain(std::iter::from_fn(move || {
            let third_item = first_item.clone() + second_item.clone();
            first_item = second_item.clone();
            second_item = third_item.clone();
            Some(third_item)
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fibinacci_index_by_length_3() {
        let result = calculate_fibinacci_index_by_length(3);

        assert_eq!(result, Some(12));
    }
}
