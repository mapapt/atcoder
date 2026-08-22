use std::prelude::rust_2024::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    rc::*, cell::*,
};

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ( $($x:tt)* ) => {};
}

#[cfg(debug_assertions)]
macro_rules! debug {
    () => {
        eprintln!("[@{}]", line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            ref tmp => {
                eprintln!("[@{}] {} = {:?}",
                    line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(debug!($val)),+,)
    };
}

struct StdIo<'a> {
    tokens: std::str::SplitWhitespace<'a>,
    delim: Option<bool>,
    en_delim: bool,
}

#[allow(dead_code)]
impl<'a> StdIo<'a> {
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        StdIo {
            tokens: placeholder.split_whitespace(),
            delim: None,
            en_delim: true,
        }
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        StdIo {
            tokens: placeholder.split_whitespace(),
            delim: None,
            en_delim: true,
        }
    }
    fn en_delim(&mut self, en: bool) {
        self.en_delim = en;
    }
    fn next_string(&mut self) -> String {
        self.tokens.next().unwrap().to_string()
    }
    fn next_bytes(&mut self) -> Vec<u8> {
        self.tokens.next().unwrap().as_bytes().to_vec()
    }
    fn next<T>(&mut self) -> T
    where T: std::str::FromStr, T::Err: std::fmt::Debug {
        self.tokens.next().unwrap().parse().unwrap()
    }
    fn collect<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<T> {
        (0..n).map(|_| self.next()).collect()
    }
    fn put<T>(&mut self, val: T)
    where T: std::fmt::Display {
        if let Some(delim) = self.delim {
            if delim || self.en_delim {
                std::io::stdout().write_fmt(format_args!(" ")).unwrap();
            }
        }
        std::io::stdout().write_fmt(format_args!("{}", val)).unwrap();
        self.delim = if self.en_delim {Some(true)} else {Some(false)};
    }
    fn puti<A, T>(&mut self, val: A)
    where A: AsRef<[T]>, T: std::fmt::Display {
        for i in val.as_ref() {
            self.put(i);
        }
    }
    fn putn(&mut self) {
        std::io::stdout().write_fmt(format_args!("\n")).unwrap();
        std::io::stdout().flush().unwrap();
        self.delim = None;
    }
    fn puty(&mut self, yes: bool) {
        if yes {
            self.put("Yes");
        }
        else {
            self.put("No");
        }
        self.putn();
    }
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let n: usize = io.next();
    let q: usize = io.next();
    let xy: Vec<(i64, i64)> = (0..n).map(|_| (io.next(), io.next())).collect();
    let mut sxy = Vec::new();
    let mut sx = 0;
    let mut sy = 0;
    sxy.push((sx, sy));
    for (x, y) in xy.iter() {
        sx += x;
        sy += y;
        sxy.push((sx, sy));
    }
    let mut rxy = Vec::new();
    let mut sx = 0;
    let mut sy = 0;
    rxy.push((sx, sy));
    for (x, y) in xy.iter().rev() {
        sx += x;
        sy += y;
        rxy.push((sx, sy));
    }
    for _ in 0..q {
        let u: usize = io.next();
        let v: usize = io.next();
        let u = u - 1;
        let v = v - 1;
        if v > u {
            let dx = sxy[v + 1].0 - sxy[u].0;
            let dy = sxy[v + 1].1 - sxy[u].1;
            let x = dx as f64 / (v.abs_diff(u) + 1) as f64;
            let y = dy as f64 / (v.abs_diff(u) + 1) as f64;
            io.put(x);
            io.put(y);
        }
        else {
            let dx = rxy[u + 1].0 - rxy[v].0;
            let dy = rxy[u + 1].1 - rxy[v].1;
            let x = dx as f64 / (v.abs_diff(u) + 1) as f64;
            let y = dy as f64 / (v.abs_diff(u) + 1) as f64;
            io.put(x);
            io.put(y);
        }
        io.putn();
    }
}
