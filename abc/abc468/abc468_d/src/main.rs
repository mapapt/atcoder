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

#[derive(Debug, Clone)]
struct CombIter
{
    n: usize,
    k: usize,
}

#[derive(Debug, Clone)]
struct IterComb<'a>
{
    comb: &'a CombIter,
    n_c_k: Vec<usize>,
    first: bool,
    end: bool,
}

impl CombIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        CombIter {n, k}
    }

    fn iter(&self) -> IterComb<'_> {
        let n_c_k = (0..self.k).collect();
        IterComb {
            comb: &self,
            n_c_k,
            first: true, end: false
        }
    }
}

impl<'a> Iterator for IterComb<'a>
{
    type Item = &'a[usize];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            if self.comb.k == 0 {
                self.end = true;
            }
        }
        else if self.end {
            return None;
        }
        else {
            let n = self.comb.n;
            let k = self.comb.k;

            for pos in (0..k).rev() {
                let c = self.n_c_k[pos] + 1;
                if c < n - k + 1 + pos {
                    for i in pos..k {
                        self.n_c_k[i] = c + i - pos;
                    }
                    break;
                }
                else {
                    if pos == 0 {
                        self.end = true;
                        return None;
                    }
                }
            }
        }
    
        Some(
            unsafe {
                // self is borrowed as `&mut`, but this returns its contents as `&`.
                // It violates `&mut` constraints.
                std::mem::transmute(self.n_c_k.as_slice())
            }
        )
    }
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let s = io.next_string(); // String
    let s: Vec<char> = s.chars().collect();
    let n = s.len();

    let mut cnt = 0;
    for i in 0..n {
        let mut miss = 0;
        for j in 0.. {
            if i < j || i + j >= n {
                break;
            }
            debug!(i, j);
            if s[i - j] == s[i + j] {
                cnt += 1;
            }
            else if miss < 1 {
                cnt += 1;
                miss += 1;
            }
            else {
                break;
            }
        }
    }
    for i in 0..(n - 1) {
        let mut miss = 0;
        for j in 0.. {
            if i < j || i + j + 1 >= n {
                break;
            }
            debug!(i, j);
            if s[i - j] == s[i + j + 1] {
                cnt += 1;
            }
            else if miss < 1 {
                cnt += 1;
                miss += 1;
            }
            else {
                break;
            }
        }
    }
    io.put(cnt);
    io.putn();
}
