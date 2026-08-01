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
    let mut ab = HashSet::new();
    for _ in 0..m {
        let a: usize = io.next();
        let b: usize = io.next();
        ab.insert((a - 1, b - 1));
    }
    let ab = ab;
    debug!(ab);

    if ab.len() == 1 {
        let x = (n - 1) * (n - 2);
        debug!();
        io.put(x);
        io.putn();
        return;
    }

    let p0 = ab.iter().next().unwrap();
    let p0 = *p0;
    let mut p1 = None;
    let mut p2 = None;
    for abi in ab.iter() {
        if p0.0 != abi.0 && p0.0 != abi.1 && p0.1 != abi.0 && p0.1 != abi.1 {
            p1 = Some(*abi);
            break;
        }
        else if p0 != *abi {
            p2 = Some(*abi);
        }
    }

    if let Some(p1) = p1 {
        let mut s = HashSet::new();
        s.insert((p0.0, p1.0));
        s.insert((p0.0, p1.1));
        s.insert((p0.1, p1.0));
        s.insert((p0.1, p1.1));

        for abi in ab.iter() {
            let ss = s.clone();
            for si in ss.iter() {
                if si.0 != abi.0 && si.0 != abi.1 && si.1 != abi.0 && si.1 != abi.1 {
                    s.remove(si);
                }
            }
        }
        io.put(s.len());
        io.putn();
        return;
    }
    else if let Some(p2) = p2 {
        let p3 = if p0.0 == p2.0 {
            (p0.1, p2.1)
        }
        else if p0.0 == p2.1 {
            (p0.1, p2.0)
        }
        else if p0.1 == p2.0 {
            (p0.0, p2.1)
        }
        else {
            (p0.0, p2.0)
        };

        let x = n - 1;
        for abi in ab.iter() {
            if p3.0 != abi.0 && p3.0 != abi.1 && p3.1 != abi.0 && p3.1 != abi.1 {
                io.put(x);
                io.putn();
                return;
            }
        }
        io.put(x + 1);
        io.putn();
        return;
    }
    else {
        let x = (n - 1) * (n - 2);
        io.put(x);
        io.putn();
        return;
    }
}
