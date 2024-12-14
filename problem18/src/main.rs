fn main() {
    let triangle = Triangle::from(vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 4, 82, 47, 65],
        vec![19, 1, 23, 75, 3, 34],
        vec![88, 2, 77, 73, 7, 63, 67],
        vec![99, 65, 4, 28, 6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ]);
    let result = triangle.max_path_sum();
    match result {
        Some(sum) => println!("{}", sum),
        None => println!("The sum on the paths could not be calculated."),
    }
}

struct Triangle {
    rows: Vec<Row>,
}

struct Row {
    numbers: Vec<u64>,
}

struct Sum {
    numbers: Vec<u64>,
}

impl Triangle {
    fn from(rows: Vec<Vec<u64>>) -> Triangle {
        Triangle {
            rows: rows.into_iter().map(Row::new).collect(),
        }
    }

    fn max_path_sum(&self) -> Option<u64> {
        let mut sum = Sum::new(vec![]);
        for row in &self.rows {
            sum = row.add_to_sum(sum);
        }
        sum.max()
    }
}

impl Row {
    fn new(numbers: Vec<u64>) -> Row {
        Row { numbers }
    }

    fn add_to_sum(&self, sum: Sum) -> Sum {
        let mut result = Sum::new(self.numbers.clone());
        result.add_first(&sum);
        result.add_middle(&sum);
        result.add_last(&sum);
        result
    }
}

impl Sum {
    fn new(numbers: Vec<u64>) -> Sum {
        Sum { numbers }
    }

    fn add_first(&mut self, previous_sum: &Sum) {
        if previous_sum.len() == 0 {
            return;
        }
        self.numbers[0] += previous_sum.numbers[0];
    }

    fn add_last(&mut self, previous_sum: &Sum) {
        if self.len() == 1 {
            return;
        }
        let last = self.len() - 1;
        self.numbers[last] += previous_sum.numbers[previous_sum.len() - 1];
    }

    fn add_middle(&mut self, previous_sum: &Sum) {
        if previous_sum.len() < 2 {
            return;
        }
        (1..=self.len() - 2).for_each(|i| {
            self.numbers[i] += previous_sum.numbers[i - 1].max(previous_sum.numbers[i]);
        });
    }

    fn len(&self) -> usize {
        self.numbers.len()
    }

    fn max(&self) -> Option<u64> {
        self.numbers.iter().max().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_path_sum() {
        let triangle = Triangle::from(vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]]);
        assert_eq!(triangle.max_path_sum(), Some(23));
    }
}
