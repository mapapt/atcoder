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
    let a: Vec<i64> = io.collect(n);

    let mut suma = Vec::new();
    let mut sum = 0;
    suma.push(sum);
    for ai in a.iter() {
        sum += ai;
        suma.push(sum);
    }
    debug!(suma);

    let mut blen = Vec::new();
    let mut len = 0;
    for i in 0..n {
        len += i + 1;
        blen.push(len);
    }
    debug!(blen);

    for _ in 0..q {
        let l: usize = io.next();
        let r: usize = io.next();
        let l = l - 1;
        let r = r - 1;

        let lbi = blen.partition_point(|&x| x <= l);
        let rbi = blen.partition_point(|&x| x <= r);
        debug!(lbi, rbi);

        let ls = l - if lbi > 0 {blen[lbi - 1]} else {0};
        let le = lbi;
        debug!(ls, le);

        let rs = 0;
        let re = r - if rbi > 0 {blen[rbi - 1]} else {0};
        debug!(rs, re);

        let mut ans;

        if lbi == rbi {
            ans = suma[re + 1] - suma[ls];
        }
        else {
            ans = (suma[le + 1] - suma[ls]) + (suma[re + 1] - suma[rs]);
            if rbi - lbi > 1 {
                let cbi = (rbi + lbi) / 2;
                debug!(rbi - lbi, cbi);
                if (rbi - lbi) % 2 == 0 {
                    ans += suma[cbi + 1] * ((rbi - lbi - 1) as i64);
                }
                else {
                    ans += suma[cbi + 1] * ((rbi - lbi - 2) as i64) + suma[rbi];
                }
            }
        }

        io.put(ans);
        io.putn();
    }
}
