use std::f64::consts::PI;
use std::ops::Range;

use comp_math::math::{factorial::*, polynomials::*};
use comp_math::polynomial;
use prettytable::Table;

struct LagrangePolynomial<Func, Derivative>
where
    Func: Fn(f64) -> f64,
    Derivative: Fn(usize) -> Box<dyn Fn(f64) -> f64>,
{
    func: Func,
    derivative: Derivative,
    interval: Range<f64>,
    fact_cache: Fact,
}

impl<Func, Derivative> LagrangePolynomial<Func, Derivative>
where
    Func: Fn(f64) -> f64,
    Derivative: Fn(usize) -> Box<dyn Fn(f64) -> f64>,
{
    pub fn new(func: Func, derivative: Derivative, interval: Range<f64>) -> Self {
        Self {
            func,
            derivative,
            interval,
            fact_cache: Fact::new(),
        }
    }

    fn calculate_points(&self, n: usize) -> Result<Vec<f64>, String> {
        if n <= 1 {
            return Err("N in calculate_points can't be less then 2 (because we already have 2 points — start of interval (a) and end of interval (b)".into());
        };
        let (a, b) = (self.interval.start, self.interval.end);
        let step = (b - a).abs() / (n - 1) as f64;
        let mut points = vec![0.; n];
        for i in 0..n {
            points[i] = a + step * i as f64
        }
        Ok(points)
    }

    fn lagrange_polynomial(&self, n: usize) -> Result<Polynomial, String> {
        let mut sum = polynomial!();
        let points = self.calculate_points(n)?;
        let mut v = vec![0.; n];
        for i in 0..n {
            v[i] = (self.func)(points[i]);
            let mut mul = polynomial!((self.func)(points[i]));
            for j in 0..n {
                if i != j {
                    mul *= polynomial!(-points[j], 1.) / (points[i] - points[j]);
                }
            }
            sum += mul;
        }
        Ok(sum)
    }

    fn norm_of_function(&self, func: &dyn Fn(f64) -> f64, point: Option<f64>) -> f64 {
        if let Some(point) = point {
            return func(point).abs();
        }

        let (a, b) = (self.interval.start, self.interval.end);
        let (mut x0, mut max, eps) = (a, 0., 0.0001);
        while x0 < b {
            x0 += eps;
            let y0 = func(x0).abs();
            max = f64::max(max, y0)
        }
        max
    }

    fn absolute_error(&self, n: usize, point: Option<f64>) -> f64 {
        let polynom = self.lagrange_polynomial(n).unwrap();
        self.norm_of_function(&|x| polynom.calculate_at(x) - (self.func)(x), point)
    }

    fn relative_error(&self, n: usize, point: Option<f64>) -> f64 {
        self.absolute_error(n, point) / self.norm_of_function(&self.func, point) * 100.
    }

    fn lagrange_error_bound(&mut self, n: usize, point: Option<f64>) -> f64 {
        let actual_derivative = (self.derivative)(n);
        let Range { start: a, end: b } = self.interval;
        self.fact_cache.fact(n);
        self.norm_of_function(&actual_derivative, point) / self.fact_cache[n].unwrap()
            * (b - a).powi(n as i32)
    }

    pub fn statistics_about_max_error(&mut self, degrees_of_polynomials: &[usize]) -> Vec<Record> {
        let mut records = vec![];

        for n in degrees_of_polynomials {
            let n = *n;
            let abs_error = self.absolute_error(n, None);
            let rel_error = self.relative_error(n, None);
            let lagrange_error = self.lagrange_error_bound(n, None);
            records.push(Record::new(n, abs_error, rel_error, lagrange_error));
        }

        records
    }

    pub fn statistics_about_error_in_point(
        &mut self,
        degrees_of_polynomials: &[usize],
        point: f64,
    ) -> Vec<Record> {
        let mut records = vec![];

        for n in degrees_of_polynomials {
            let n = *n;
            let abs_error = self.absolute_error(n, Some(point));
            let rel_error = self.relative_error(n, Some(point));
            let lagrange_error = self.lagrange_error_bound(n, Some(point));
            records.push(Record::new(n, abs_error, rel_error, lagrange_error));
        }

        records
    }
}

#[derive(Debug, Clone)]
struct Record {
    pub n: usize,
    pub abs_error: f64,
    pub rel_error: f64,
    pub lagrange_error: f64,
}

impl Record {
    pub fn new(n: usize, abs_error: f64, rel_error: f64, lagrange_error: f64) -> Self {
        Record {
            n,
            abs_error,
            rel_error,
            lagrange_error,
        }
    }
}

use prettytable::row;

fn main() {
    println!(
        "{}",
        if dbg!(rand::random::<i32>()) % -1 == 0 {
            1
        } else {
            -1
        }
    );
    let func = |x: f64| x.powi(2) - (PI * x).sin();

    let derivative = |n: usize| {
        let inner = move |x: f64| {
            let theta = match n {
                1 => 2. * x,
                2 => 2.,
                _ => 0.,
            };
            let n = n as _;

            theta - PI.powf(n) * (PI * x + PI / 2. * n).sin()
        };
        let inner: Box<dyn Fn(f64) -> f64> = Box::new(inner);

        inner
    };

    let exp = |x| format!("{x:e}");

    let interval = 0.4_f64..0.9_f64;
    let mut app = LagrangePolynomial::new(func, derivative, interval);

    let n = [3, 5, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let points = [0.53, 0.43, 0.86, 0.67];
    let mut table = Table::new();

    table.add_row(row!["x", "n", "Δƒₙ", "δƒₙ", "rₙ"]);

    for x in points {
        let records = app.statistics_about_error_in_point(&n, x);
        let mut table = table.clone();
        for record in records {
            let Record {
                n,
                abs_error,
                rel_error,
                lagrange_error,
            } = record;
            table.add_row(row![
                x,
                n,
                exp(abs_error),
                exp(rel_error),
                exp(lagrange_error)
            ]);
        }
        table.printstd();
    }

    let mut table = Table::new();
    table.add_row(row!["n", "Δƒₙ", "δƒₙ", "rₙ"]);
    let records = app.statistics_about_max_error(&n);
    for record in records {
        let Record {
            n,
            abs_error,
            rel_error,
            lagrange_error,
        } = record;
        table.add_row(row![n, exp(abs_error), exp(rel_error), exp(lagrange_error)]);
    }
    table.printstd();
}
