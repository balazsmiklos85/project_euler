pub struct Sum {
    numbers: Vec<u64>,
}

impl Sum {
    pub fn new(numbers: Vec<u64>) -> Sum {
        Sum { numbers }
    }

    pub fn add_first(&mut self, previous_sum: &Sum) {
        if previous_sum.len() == 0 {
            return;
        }
        self.numbers[0] += previous_sum.numbers[0];
    }

    pub fn add_last(&mut self, previous_sum: &Sum) {
        if self.len() == 1 {
            return;
        }
        let last = self.len() - 1;
        self.numbers[last] += previous_sum.numbers[previous_sum.len() - 1];
    }

    pub fn add_middle(&mut self, previous_sum: &Sum) {
        if previous_sum.len() < 2 {
            return;
        }
        (1..=self.len() - 2).for_each(|i| {
            self.numbers[i] += previous_sum.numbers[i - 1].max(previous_sum.numbers[i]);
        });
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    pub fn max(&self) -> Option<u64> {
        self.numbers.iter().max().copied()
    }
}
