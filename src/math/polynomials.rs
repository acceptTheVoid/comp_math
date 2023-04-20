use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone)]
pub struct Polynomial {
    data: Vec<f64>,
}

impl Polynomial {
    pub fn new(size: usize) -> Polynomial {
        Self {
            data: vec![0.; size + 1],
        }
    }

    pub fn degree(&self) -> usize {
        self.data.len() - 1
    }

    pub fn calculate_at(&self, x0: f64) -> f64 {
        let mut y0 = 0.;
        for i in 0..=self.degree() {
            y0 += x0.powi(i as _) * self[i]
        }
        y0
    }
}

impl From<Vec<f64>> for Polynomial {
    fn from(vec: Vec<f64>) -> Self {
        if vec.is_empty() {
            Self { data: vec![0.] }
        } else {
            Self { data: vec }
        }
    }
}

#[macro_export]
macro_rules! polynomial {
    ( $($x:expr),* ) => {
        Polynomial::from(vec![$($x),*])
    };
}

impl Index<usize> for Polynomial {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Polynomial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;
    fn neg(mut self) -> Self::Output {
        self *= -1.;
        self
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        let mut tmp = Vec::new();
        let (mut data_max, data_min) = if self.degree() > rhs.degree() {
            std::mem::swap(&mut self.data, &mut tmp);
            (tmp, &rhs.data)
        } else {
            (rhs.data, &self.data)
        };

        for (l_coefficient, r_coefficient) in data_max.iter_mut().zip(data_min) {
            *l_coefficient += r_coefficient;
        }

        self.data = data_max;
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}

impl Mul<f64> for Polynomial {
    type Output = Polynomial;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Polynomial {
    fn mul_assign(&mut self, rhs: f64) {
        for coefficient in &mut self.data {
            *coefficient *= rhs
        }
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;
    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f64> for Polynomial {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        let (l_degree, r_degree) = (self.degree(), rhs.degree());
        let mut mul = Self::new(l_degree + r_degree);
        for i in 0..=l_degree {
            for j in 0..=r_degree {
                mul[i + j] += self[i] * rhs[j];
            }
        }

        *self = mul;
    }
}
