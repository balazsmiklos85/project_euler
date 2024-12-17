fn main() {
    let permutations = Permutations {
        of: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    };
    let permutation = permutations.get(1000000 - 1);
    println!("{}", permutation);
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

struct Permutations {
    of: Vec<u8>,
}

impl Permutations {
    fn calculate_factorial_representation(&self, index: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut to_slice_up = index;
        for factorial_component in (1..self.of.len()).rev() {
            let lehmer_index = to_slice_up / factorial(factorial_component);
            to_slice_up %= factorial(factorial_component);
            result.push(lehmer_index);
        }
        result.push(0);
        result
    }

    fn get(&self, index: usize) -> String {
        let lehmer_code = self.calculate_factorial_representation(index);
        let permutation = self.get_permutation_from_lehmer_code(lehmer_code);
        permutation
            .into_iter()
            .map(|digit| digit.to_string())
            .collect()
    }

    fn get_permutation_from_lehmer_code(&self, lehmer_code: Vec<usize>) -> Vec<u8> {
        let mut range = self.of.clone();
        lehmer_code
            .into_iter()
            .map(|lehmer_index| range.remove(lehmer_index))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        let given = Permutations { of: vec! [0, 1, 2] };

        assert_eq!(given.get(0), "012");
        assert_eq!(given.get(1), "021");
        assert_eq!(given.get(2), "102");
        assert_eq!(given.get(3), "120");
        assert_eq!(given.get(4), "201");
        assert_eq!(given.get(5), "210");
    }

}
