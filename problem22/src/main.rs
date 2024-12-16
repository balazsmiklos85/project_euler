use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let input = read_raw_names("problem22/src/names.txt");
    let result = input
        .into_iter()
        .enumerate()
        .map(|(index, name)| parse(index, name))
        .map(|name| name.score())
        .sum::<u64>();
    println!("{}", result);
}

fn alphabet_index(c: char) -> u64 {
    match c {
        'A'..='Z' => (c as u64) - ('A' as u64) + 1,
        _ => panic!("Unexpected character: {}", c),
    }
}

fn parse(index: usize, name: String) -> Name {
    Name::from(index + 1, name)
}

fn read_raw_names(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut raw_names = reader
        .lines()
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split(',')
                .map(|name| name.to_string())
                .map(|name| name.replace("\"", ""))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();
    raw_names.sort();
    raw_names
}

struct Name {
    index: usize,
    value: u64,
}

impl Name {
    fn from(index: usize, name: String) -> Name {
        let value = name.chars().map(alphabet_index).sum();
        Name { index, value }
    }

    fn score(&self) -> u64 {
        self.index as u64 * self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_of_colin() {
        let given = Name::from(938, "COLIN".to_string());
        let result = given.score();
        assert_eq!(given.index, 938);
        assert_eq!(given.value, 53);
        assert_eq!(result, 49714);
    }
}
