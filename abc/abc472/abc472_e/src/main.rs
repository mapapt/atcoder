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

fn sub(g: &Vec<BTreeSet<usize>>, v: &mut Vec<Option<usize>>, i: usize, cnt: usize) -> Option<Vec<usize>> {
    if let Some(vc) = v[i] {
        if vc.abs_diff(cnt) % 2 == 1 {
            return Some(vec![i]);
        }
    }
    else {
        v[i] = Some(cnt);
        for &j in g[i].iter() {
            if let Some(mut ans) = sub(g, v, j, cnt + 1) {
                if ans.len() < 2 || ans.first().unwrap() != ans.last().unwrap() {
                    ans.push(i);
                }
                return Some(ans);
            }
        }
    }
    None
}

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let t: usize = io.next();
    for _ in 0..t {
        let n: usize = io.next();
        let m: usize = io.next();
        let mut g = vec![BTreeSet::new(); n];
        for _ in 0..m {
            let a: usize = io.next();
            let b: usize = io.next();
            let a = a - 1;
            let b = b - 1;
            g[a].insert(b);
            g[b].insert(a);
        }
        debug!(g);

        let mut v = vec![None; n];
        if let Some(ans) = sub(&g, &mut v, 0, 0) {
            io.put(ans.len() - 1);
            io.putn();
            for k in 0..(ans.len() - 1) {
                io.put(ans[k] + 1);
            }
            io.putn();
        }
        else {
            io.put(-1);
            io.putn();
        }
    }
}
