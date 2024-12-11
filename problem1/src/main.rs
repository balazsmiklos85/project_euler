fn main() {
    let result = sum_multiples_of_3_or_5(1000);
    println!("{}", result);
}

fn sum_multiples_of_3_or_5(limit: u32) -> u32 {
    (0..limit).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiples_of_3_or_5() {
        assert_eq!(sum_multiples_of_3_or_5(10), 23);
    }
}
