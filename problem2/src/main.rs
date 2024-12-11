fn main() {
    let result = custom_fibonacci_sequence(1, 2)
        .filter(|x| x % 2 == 0)
        .take_while(|x| x < &4_000_000)
        .sum::<u64>();
    println!("{}", result);
}

fn custom_fibonacci_sequence(
    mut first_item: u64,
    mut second_item: u64,
) -> impl Iterator<Item = u64> {
    std::iter::once(first_item)
        .chain(std::iter::once(second_item))
        .chain(std::iter::from_fn(move || {
            let third_item = first_item + second_item;
            first_item = second_item;
            second_item = third_item;
            Some(third_item)
        }))
}

#[test]
fn test_custom_fibonacci_sequence() {
    assert_eq!(
        custom_fibonacci_sequence(1, 2)
            .take(10)
            .collect::<Vec<u64>>(),
        vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
    );
}
