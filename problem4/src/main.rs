use num::pow;

fn main() {
    let length = 3;
    let result = find_largest_palindrome_by_factor_length(length);
    if let Some(palindrome) = result {
        println!("{}", palindrome);
    } else {
        println!("No palindromes found");
    }
}

fn find_largest_palindrome_by_factor_length(length: usize) -> Option<u64> {
    let upper_limit = pow(10, length) - 1;
    let first_item = pow(10, length - 1);
    (first_item..=upper_limit)
        .rev()
        .flat_map(|number| {
            (number..=upper_limit)
                .rev()
                .map(move |other| number * other)
        })
        .filter(|number| is_palindrome(number))
        .max()
}

fn is_palindrome(number: &u64) -> bool {
    let as_string = number.to_string();
    as_string == as_string.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_largest_palindrome_by_factor_length_2() {
        assert_eq!(find_largest_palindrome_by_factor_length(2), Some(9009));
    }

    #[test]
    fn is_palindrome_finds_palindrome() {
        assert!(is_palindrome(&121));
    }

    #[test]
    fn is_palindrome_finds_not_palindrome() {
        assert!(!is_palindrome(&123));
    }
}
