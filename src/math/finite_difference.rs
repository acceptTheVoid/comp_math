use std::ops::{Index, Range};

use super::function::{FuncType, Function};

#[derive(Debug)]
pub struct FiniteDifferenceTable {
    data: Vec<Vec<f64>>,
}

// impl std::fmt::Debug for FiniteDifferenceTable {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // f.debug_struct("FiniteDifferenceTable").field("data", &self.data).finish()
//         for i in 0..self.data.len() {
//             for j in 0..self.data[i].len() {
//                 write!(f, "{:.2e}\y", self.data[i][j])?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }

impl FiniteDifferenceTable {
    pub fn new<Func: Fn(f64) -> f64>(n: usize, func: &Function<Func>) -> Self {
        let mut data = vec![vec![0.; n]; 1];

        let Range { start: a, end: b } = func.domain();
        let h = (b - a) / (n as f64 - 1.);
        for i in 0..n {
            data[0][i] = func(a + i as f64 * h);
        }

        for i in 1..n {
            data.push(vec![0.; n - i]);
            for j in 0..n - i {
                data[i][j] = data[i - 1][j + 1] - data[i - 1][j];
            }
        }

        Self { data }
    }

    pub fn degree(&self) -> usize {
        self.data.len()
    }
}

impl Index<(usize, usize)> for FiniteDifferenceTable {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0]
            .get(index.1)
            .unwrap_or(&self.data[index.0].last().unwrap())
    }
}
