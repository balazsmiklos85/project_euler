use crate::{row::Row, sum::Sum};

pub struct Triangle {
    rows: Vec<Row>,
}

impl Triangle {
    pub fn from(rows: Vec<Vec<u64>>) -> Triangle {
        Triangle {
            rows: rows.into_iter().map(Row::new).collect(),
        }
    }

    pub fn max_path_sum(&self) -> Option<u64> {
        let mut sum = Sum::new(vec![]);
        for row in &self.rows {
            sum = row.add_to_sum(sum);
        }
        sum.max()
    }
}

#[cfg(test)]
mod tests {
    use crate::triangle::Triangle;

    #[test]
    fn test_max_path_sum() {
        let triangle = Triangle::from(vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]]);
        assert_eq!(triangle.max_path_sum(), Some(23));
    }
}
