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

fn sub(a: &[u32], b: &[u32]) -> Option<usize> {
    for i in 0.. {
        if i < a.len() && i < b.len() {
            if a[i] != b[i] {
                return Some(i);
            }
        }
        else {
            break;
        }
    }
    None
}

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let n: usize = io.next();
    let a: Vec<u32> = io.collect(n);
    let b: Vec<u32> = io.collect(n);

    let ans = if let Some(p0) = sub(&a, &b) {
        let ans_a = if let Some(p1) = sub(a.split_at(p0 + 1).1, b.split_at(p0).1) {
            if a[p0] == b[p0 + p1] {
                if let Some(_) = sub(a.split_at(p0 + p1 + 1).1, b.split_at(p0 + p1 + 1).1) {
                    false
                }
                else {
                    debug!();
                    true
                }
            }
            else {
                false
            }
        }
        else {
            debug!();
            a[p0] == b[n - 1]
        };
        debug!(ans_a);

        let ans_b = if let Some(p1) = sub(b.split_at(p0 + 1).1, a.split_at(p0).1) {
            if b[p0] == a[p0 + p1] {
                if let Some(_) = sub(b.split_at(p0 + p1 + 1).1, a.split_at(p0 + p1 + 1).1) {
                    false
                }
                else {
                    debug!();
                    true
                }
            }
            else {
                false
            }
        }
        else {
            debug!();
            b[p0] == a[n - 1]
        };
        debug!(ans_b);

        ans_a || ans_b
    }
    else {
        debug!();
        true
    };

    io.puty(ans);
}
