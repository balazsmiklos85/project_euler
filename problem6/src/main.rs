fn main() {
    let sum_of_squares = calculate_sum_of_squares(100);
    let square_of_sums = calculate_square_of_sums(100);
    let difference = square_of_sums - sum_of_squares;
    println!("{}", difference);
}

fn calculate_sum_of_squares(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn calculate_square_of_sums(n: u64) -> u64 {
    (n * (n + 1) / 2).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum_of_squares() {
        assert_eq!(calculate_sum_of_squares(10), 385);
    }

    #[test]
    fn test_calculate_square_of_sums() {
        assert_eq!(calculate_square_of_sums(10), 3025);
    }
}
