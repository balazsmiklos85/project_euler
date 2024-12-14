use crate::sum::Sum;

pub struct Row {
    numbers: Vec<u64>,
}

impl Row {
    pub fn new(numbers: Vec<u64>) -> Row {
        Row { numbers }
    }

    pub fn add_to_sum(&self, sum: Sum) -> Sum {
        let mut result = Sum::new(self.numbers.clone());
        result.add_first(&sum);
        result.add_middle(&sum);
        result.add_last(&sum);
        result
    }
}
