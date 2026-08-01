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
    let m: usize = io.next();
    let mut ab = BTreeSet::new();
    for _ in 0..m {
        let a: usize = io.next();
        let b: usize = io.next();
        ab.insert((a - 1, b - 1));
    }
    debug!(ab);

    let ab0 = ab.pop_first().unwrap();

    let c0 = ab0.0;
    let mut f0 = true;
    let mut s0 = HashSet::new();
    for abi in ab.iter() {
        if c0 != abi.0 && c0 != abi.1 {
            if f0 {
                f0 = false;
                s0.insert(abi.0);
                s0.insert(abi.1);
            }
            else {
                let ss = s0.clone();
                s0.clear();
                for &ssi in ss.iter() {
                    if ssi == abi.0 || ssi == abi.1 {
                        s0.insert(ssi);
                    }
                }
            }
            debug!(s0);
        }
    }

    let c1 = ab0.1;
    let mut f1 = true;
    let mut s1 = HashSet::new();
    for abi in ab.iter() {
        if c1 != abi.0 && c1 != abi.1 {
            if f1 {
                f1 = false;
                s1.insert(abi.0);
                s1.insert(abi.1);
            }
            else {
                let ss = s1.clone();
                s1.clear();
                for &ssi in ss.iter() {
                    if ssi == abi.0 || ssi == abi.1 {
                        s1.insert(ssi);
                    }
                }
            }
            debug!(s1);
        }
    }

    debug!(f0, f1, s0, s1);
    if f0 && f1 {
        io.put((n - 1) + (n - 2));
        io.putn();
    }
    else if f0 {
        s1.remove(&c0);
        io.put((n - 1) + s1.len());
        io.putn();
    }
    else if f1 {
        s0.remove(&c1);
        io.put((n - 1) + s0.len());
        io.putn();
    }
    else {
        let mut s01 = HashSet::new();
        for &s0i in s0.iter() {
            s01.insert(if c0 < s0i {(c0, s0i)} else {(s0i, c0)});
        }
        for &s1i in s1.iter() {
            s01.insert(if c1 < s1i {(c1, s1i)} else {(s1i, c1)});
        }
        io.put(s01.len());
        io.putn();
    }
}
