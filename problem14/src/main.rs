fn main() {
    let result = (2..1_000_000)
        .map(|n| calculate_collatz_length(n))
        .max_by_key(|collatz| collatz.length);

    match result {
        Some(collatz) => println!("{}", collatz.start),
        None => println!("No result"),
    }
}

struct Collatz {
    start: u64,
    length: u64,
}

fn calculate_collatz_length(starting_number: u64) -> Collatz {
    let mut length = 0;
    let mut current_number = starting_number;
    while current_number != 1 {
        if current_number % 2 == 0 {
            current_number /= 2;
        } else {
            current_number = 3 * current_number + 1;
        }
        length += 1;
    }
    Collatz {
        start: starting_number,
        length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_collatz_length() {
        assert_eq!(calculate_collatz_length(13).length, 10);
    }
}
