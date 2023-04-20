use std::ops::Index;

#[derive(Debug)]
pub struct Fact([Option<f64>; 102]);

impl Fact {
    pub fn new() -> Self {
        let mut arr = [None; 102];
        arr[0] = Some(1.);
        Self(arr)
    }

    pub fn fact(&mut self, n: usize) -> f64 {
        assert!(n < 102);
        let at_n = self.0[n];
        match at_n {
            Some(res) => res,
            None => {
                let res = self.fact(n - 1) * n as f64;
                self.0[n] = Some(res);
                res
            }
        }
    }
}

impl Index<usize> for Fact {
    type Output = Option<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
