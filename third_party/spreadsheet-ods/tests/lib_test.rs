#![allow(dead_code, unreachable_pub)]

use spreadsheet_ods::{OdsError, WorkBook};
use std::fmt::{Display, Formatter};
use std::fs;
use std::hint::black_box;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::Instant;

pub fn init_test() -> Result<(), OdsError> {
    fs::create_dir_all("test_out")?;
    Ok(())
}

pub fn test_write_ods<P: AsRef<Path>>(book: &mut WorkBook, ods_path: P) -> Result<(), OdsError> {
    fs::create_dir_all("test_out")?;
    spreadsheet_ods::write_ods(book, ods_path)
}

pub fn test_write_odsbuf(book: &mut WorkBook) -> Result<Vec<u8>, OdsError> {
    fs::create_dir_all("test_out")?;
    spreadsheet_ods::write_ods_buf(book, Vec::new())
}

#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
}

impl Unit {
    pub fn conv(&self, nanos: f64) -> f64 {
        match self {
            Unit::Nanosecond => nanos,
            Unit::Microsecond => nanos / 1000.0,
            Unit::Millisecond => nanos / 1000000.0,
            Unit::Second => nanos / 1000000000.0,
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Unit::Nanosecond => "ns",
            Unit::Microsecond => "µs",
            Unit::Millisecond => "ms",
            Unit::Second => "s",
        };
        write!(f, "{}", v)
    }
}

#[derive(Clone, Debug)]
pub struct Timing<X = ()> {
    pub name: String,
    pub skip: usize,
    pub runs: usize,
    pub divider: u64,
    pub unit: Unit,

    /// samples in ns. already divided by divider.
    pub samples: Vec<f64>,
    pub extra: Vec<X>,
}

impl<X> Timing<X> {
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = skip;
        self
    }

    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs;
        self
    }

    pub fn divider(mut self, divider: u64) -> Self {
        self.divider = divider;
        self
    }

    pub fn unit(mut self, unit: Unit) -> Self {
        self.unit = unit;
        self
    }

    pub fn run_pp<E, R>(
        &mut self,
        mut fun: impl FnMut() -> Result<R, E>,
        mut sum: impl FnMut(Result<R, E>, &mut Vec<f64>, &mut Vec<X>) -> Result<R, E>,
    ) -> Result<R, E> {
        assert!(self.runs > 0);
        assert!(self.divider > 0);

        print!("run {} ", self.name);

        let mut bench = move || {
            let now = Instant::now();
            let result = fun();
            (now.elapsed(), result)
        };

        let mut sub_sample = Vec::new();
        let mut sub_extra = Vec::new();

        let mut n = 0;
        let mut result = loop {
            let (elapsed, result) = black_box(bench());

            n += 1;

            if n >= self.skip {
                let sample = elapsed.as_nanos() as f64 / self.divider as f64;
                sub_sample.push(sample);
            }
            if n >= self.skip + self.runs {
                break result;
            }

            let d = 10usize.pow(n.ilog10());
            if n % d == 0 {
                print!(".");
            }

            let _ = stdout().flush();
        };

        result = sum(result, &mut sub_sample, &mut sub_extra);

        self.samples.extend(sub_sample);
        self.extra.extend(sub_extra);

        println!();

        result
    }

    pub fn run_nf(&mut self, mut fun: impl FnMut()) {
        let _ = self.run_pp::<(), ()>(
            || {
                fun();
                Ok(())
            },
            |v, _s, _x| v,
        );
    }

    pub fn run<E, R>(&mut self, fun: impl FnMut() -> Result<R, E>) -> Result<R, E> {
        self.run_pp(fun, |v, _s, _x| v)
    }

    pub fn n(&self) -> usize {
        self.samples.len()
    }

    pub fn sum(&self) -> f64 {
        self.samples.iter().sum()
    }

    pub fn mean(&self) -> f64 {
        self.samples.iter().sum::<f64>() / self.samples.len() as f64
    }

    pub fn median(&self) -> (f64, f64, f64) {
        let mut s = self.samples.clone();
        s.sort_by(|v, w| v.total_cmp(w));
        let m0 = s.len() * 1 / 10;
        let m5 = s.len() / 2;
        let m9 = s.len() * 9 / 10;

        (s[m0], s[m5], s[m9])
    }

    pub fn lin_dev(&self) -> f64 {
        let mean = self.mean();
        let lin_sum = self.samples.iter().map(|v| (*v - mean).abs()).sum::<f64>();
        lin_sum / self.samples.len() as f64
    }

    pub fn std_dev(&self) -> f64 {
        let mean = self.mean();
        let std_sum = self
            .samples
            .iter()
            .map(|v| (*v - mean) * (*v - mean))
            .sum::<f64>();
        (std_sum / self.samples.len() as f64).sqrt()
    }
}

impl<X> Default for Timing<X> {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            skip: 0,
            runs: 1,
            divider: 1,
            unit: Unit::Nanosecond,
            samples: vec![],
            extra: vec![],
        }
    }
}

impl<X> Display for Timing<X> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,)?;
        writeln!(f, "{}", self.name)?;
        writeln!(f,)?;
        writeln!(
            f,
            "| n | sum | 1/10 | median | 9/10 | mean | lin_dev | std_dev |"
        )?;
        writeln!(f, "|:---|:---|:---|:---|:---|:---|:---|:---|")?;

        let n = self.n();
        let sum = self.sum();
        let (m0, m5, m9) = self.median();
        let mean = self.mean();
        let lin = self.lin_dev();
        let std = self.std_dev();

        writeln!(
            f,
            "| {} | {:.2}{} | {:.2}{} | {:.2}{} | {:.2}{} | {:.2}{} | {:.2}{} | {:.2}{} |",
            n,
            self.unit.conv(sum),
            self.unit,
            self.unit.conv(m0),
            self.unit,
            self.unit.conv(m5),
            self.unit,
            self.unit.conv(m9),
            self.unit,
            self.unit.conv(mean),
            self.unit,
            self.unit.conv(lin),
            self.unit,
            self.unit.conv(std),
            self.unit,
        )?;
        writeln!(f,)?;

        if f.alternate() {
            writeln!(f,)?;
            writeln!(f, "{}", self.name)?;
            writeln!(f,)?;
            for i in 0..self.samples.len() {
                write!(f, "| {} ", i)?;
            }
            writeln!(f, "|")?;
            for _ in 0..self.samples.len() {
                write!(f, "|:---")?;
            }
            writeln!(f, "|")?;
            for e in &self.samples {
                write!(f, "| {:.2} ", e)?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}
