fn main() {
    let numbers_as_words = numbers_as_words_upto(1000);
    let result = numbers_as_words.iter().map(|x| x.len()).sum::<usize>();
    println!("{}", result);
}

fn as_word(number: u32) -> String {
    match number {
        0..=9 => digit_as_word(number),
        10..=19 => teens_as_word(number),
        20..=99 => tens_as_word(number),
        100..=999 => hundreds_as_word(number),
        1000 => String::from("onethousand"),
        1001..=u32::MAX => panic!("Numbers are only supported up to 1000."),
    }
}

fn digit_as_word(number: u32) -> String {
    [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .get(number as usize)
    .unwrap()
    .to_string()
}

fn numbers_as_words_upto(n: u32) -> Vec<String> {
    (1..=n).map(as_word).collect::<Vec<String>>()
}

fn teens_as_word(number: u32) -> String {
    [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ]
    .get((number - 10) as usize)
    .unwrap()
    .to_string()
}

fn tens_as_word(number: u32) -> String {
    [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ]
    .get((number / 10 - 2) as usize)
    .unwrap()
    .to_string()
        + &as_word(number % 10)
}

fn hundreds_as_word(number: u32) -> String {
    let full_hundreds = digit_as_word(number / 100) + "hundred";
    if number % 100 == 0 {
        full_hundreds
    } else {
        full_hundreds + "and" + &as_word(number % 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_characters_up_to_five() {
        let numbers_as_words = numbers_as_words_upto(5);
        let result = numbers_as_words.iter().map(|x| x.len()).sum::<usize>();
        assert_eq!(result, 19);
    }

    #[test]
    fn numbers_as_words_342() {
        assert_eq!(as_word(342), "threehundredandfortytwo");
    }

    #[test]
    fn numbers_as_words_115() {
        assert_eq!(as_word(115), "onehundredandfifteen");
    }
}
