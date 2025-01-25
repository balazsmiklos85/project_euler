fn main() {
    let result = sum_multiples_of_3_or_5(1000);
    println!("{}", result);
}

fn sum_multiples_of_3_or_5(limit: u64) -> u64 {
    sum_multiples(3, &limit) + sum_multiples(5, &limit) - sum_multiples(15, &limit)
}

fn sum_multiples(n: u64, limit: &u64) -> u64 {
    if limit < &n {
        return 0;
    }

    let biggest_divisible = limit - (limit % n);
    let count = biggest_divisible / n;
    if count % 2 == 0 {
        return n * (count / 2) * (count + 1);
    }
    n * ((count + 1) / 2) * count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiples_of_3_or_5() {
        assert_eq!(sum_multiples_of_3_or_5(10), 23);
    }
}
