fn main() {
    const SUM: u64 = 1000;
    let result = pythagorean_triplets()
        .take_while(|(a, b, c)| a < &SUM || b < &SUM || c < &SUM)
        .find(|(a, b, c)| a + b + c == SUM)
        .map(|(a, b, c)| a * b * c);
    match result {
        Some(result) => println!("{}", result),
        None => println!("Could not find the Pythagorean triplet summing to 1000."),
    }
}

fn binomial_indices() -> impl Iterator<Item = (u64, u64)> {
    let mut level = 1;
    let mut x = 1;
    let mut y = 1;
    std::iter::from_fn(move || {
        let result = (x, y);
        if y == 1 {
            level += 1;
            x = 1;
            y = level;
        } else {
            x += 1;
            y -= 1;
        }
        Some(result)
    })
}

fn euclids_formula(m: u64, n: u64) -> (u64, u64, u64) {
    assert!(
        m > n,
        "Euclid's formula requires m > n, but it was {} < {}.",
        m,
        n
    );
    let m_square = m * m;
    let n_square = n * n;
    let a = m_square - n_square;
    let b = 2 * m * n;
    let c = m_square + n_square;
    (a, b, c)
}

fn pythagorean_triplets() -> impl Iterator<Item = (u64, u64, u64)> {
    binomial_indices()
        .filter(|(m, n)| m > n)
        .map(|(m, n)| euclids_formula(m, n))
}
