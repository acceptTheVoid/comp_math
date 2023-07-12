use std::{rc::Rc, f64::consts::PI};

use comp_math::math::{
    factorial::Fact,
    finite_difference::FiniteDifferenceTable,
    function::{FuncType, Function},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left = -1,
    Right = 1,
}

#[derive(Debug)]
struct InterpolationFormulas<Func: Fn(f64) -> f64> {
    func: Function<Func>,
    fdt: FiniteDifferenceTable,
    fact_cache: Fact,
}

impl<Func: Fn(f64) -> f64> InterpolationFormulas<Func> {
    pub fn new(n: usize, func: Function<Func>) -> Self {
        let fdt = FiniteDifferenceTable::new(n, &func);
        Self {
            func,
            fdt,
            fact_cache: Fact::new(),
        }
    }

    fn degree(&self) -> usize {
        self.fdt.degree()
    }

    fn newton_interpolation(&mut self, t: f64, dir: Direction) -> f64 {
        let sign = dir as i32;
        let n = self.degree();
        let mut sum = 0.;

        for i in 0..n {
            let mut mul = self.fdt[(i, if sign > 0 { 0 } else { n - i })];
            mul /= self.fact_cache.fact(i);
            
            if i == 0 {
                sum += mul;
                continue;
            }

            for j in 0..i - 1 {
                mul *= t - (j as i32 * sign) as f64
            }

            sum += mul
        }

        sum
    }

    pub fn newton_interpolation_forwards(&mut self, t: f64) -> f64 {
        self.newton_interpolation(t, Direction::Right)
    }

    pub fn newton_interpolation_backwards(&mut self, t: f64) -> f64 {
        self.newton_interpolation(t, Direction::Left)
    }

    fn gauss_interpolation(&mut self, t: f64, node_idx: usize, dir: Direction) -> f64 {
        let sign = dir as i32;
        let n = self.degree();
        let mut sum = 0.;

        for i in 0..n {
            let j = match dir {
                Direction::Left => i + 1,
                Direction::Right => i,
            } / 2;
            let j = if j > node_idx { 0 } else { node_idx - j };
            let mut f = self.fdt[(i, j)];
            f *= 1. / self.fact_cache.fact(i) as f64;

            if i == 0 {
                sum += f;
                continue;
            }

            for j in 0..i - 1 {
                let tmp = ((sign * (j as i32 + 1)) / 2) as f64;
                f *= if j % 2 != 0 { t - tmp } else { t + tmp };
            }

            sum += f;
        }

        sum
    }

    fn gauss_interpolation_forwards(&mut self, t: f64, node_idx: usize) -> f64 {
        self.gauss_interpolation(t, node_idx, Direction::Right)
    }

    fn gauss_interpolation_backwards(&mut self, t: f64, node_idx: usize) -> f64 {
        self.gauss_interpolation(t, node_idx, Direction::Left)
    }

    pub fn interpolate(&mut self, x: f64) -> Result<f64, &str> {
        let n = self.degree();
        let (a, b) = self.func.domain_tuple();

        if !self.func.domain().contains(&x) {
            return Err("la cringe")
        }

        let h = (b - a) / (n - 1) as f64;
        let tmp = (x - a) / h;
        if x <= a + h / 2. {
            return Ok(self.newton_interpolation_forwards(tmp));
        } else if x >= b + h / 2. {
            return Ok(self.newton_interpolation_backwards((x - b) / h));
        };

        let i = tmp.round() as usize;
        let t = tmp - i as f64;
        if tmp.floor() as usize == i {
            Ok(self.gauss_interpolation_forwards(t, i))
        } else {
            Ok(self.gauss_interpolation_backwards(t, i))
        }
    }
}

fn main() {
    let func = Rc::new(|x: f64| x.powi(2) - (x * PI).sin());
    let func = Function::new(Rc::clone(&func), 0.4_f64..0.9_f64);
    let mut interpol = InterpolationFormulas::new(50, func.clone());
    let points = [0.53, 0.43, 0.86, 0.67];
    for x in points {
        let res = interpol.interpolate(x).unwrap();
        println!("Point: {x}");
        println!("Function: {};\nCalculated: {res}", func(x));
        println!("Err: {}%\n", ((func(x) - res) / res * 100.).abs());
    }
}
