pub struct Sum {
    numbers: Vec<u64>,
}

impl Sum {
    pub fn new(numbers: Vec<u64>) -> Sum {
        Sum { numbers }
    }

    pub fn add_first(&mut self, previous: &Sum) {
        if let Some(first) = previous.numbers.first() {
            self.numbers[0] += first
        }
    }

    pub fn add_last(&mut self, previous: &Sum) {
        if let Some(last) = previous.numbers.last() {
            let last_index = self.len() - 1;
            self.numbers[last_index] += last;
        }
    }

    pub fn add_middle(&mut self, previous: &Sum) {
        if previous.len() < 2 {
            return;
        }
        (1..=self.len() - 2).for_each(|i| {
            self.numbers[i] += previous.numbers[i - 1].max(previous.numbers[i]);
        });
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    pub fn max(&self) -> Option<u64> {
        self.numbers.iter().max().copied()
    }
}
