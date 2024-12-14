pub struct Sum {
    numbers: Vec<u64>,
}

impl Sum {
    pub fn new(numbers: Vec<u64>) -> Sum {
        Sum { numbers }
    }

    pub fn add_first(&mut self, previous: &Sum) {
        if previous.len() == 0 {
            return;
        }
        self.numbers[0] += previous.numbers[0];
    }

    pub fn add_last(&mut self, previous: &Sum) {
        if self.len() == 1 {
            return;
        }
        let last = self.len() - 1;
        self.numbers[last] += previous.numbers[previous.len() - 1];
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
