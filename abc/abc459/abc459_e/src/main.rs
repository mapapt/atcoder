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
    delim: char,
}

#[allow(dead_code)]
impl<'a> StdIo<'a> {
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        StdIo {
            tokens: placeholder.split_whitespace(),
            delim: '\0',
        }
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        StdIo {
            tokens: placeholder.split_whitespace(),
            delim: '\0',
        }
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
        if self.delim != '\0' {
            print!("{}", self.delim);
        }
        print!("{}", val);
        self.delim = ' ';
    }
    fn puti<A, T>(&mut self, val: A)
    where A: AsRef<[T]>, T: std::fmt::Display {
        for i in val.as_ref() {
            self.put(i);
        }
    }
    fn putn(&mut self) {
        println!();
        self.delim = '\0';
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

fn comb(a: usize, b: usize) -> usize {
    if a < b {
        0
    }
    else if a == b {
        1
    }
    else {
        let b = b.min(a - b);
        let mut r = 1;
        for i in 0..b {
            r = (r * (a - i)) % 998244353;
        }
        r
    }
}
fn sub(g: &Vec<(BTreeSet<usize>, usize, usize)>, node: usize) -> (usize, usize) {
    let c = g[node].1;
    let d = g[node].2;
    if g[node].0.is_empty() {
        if c >= d {
            //debug!(node, comb(c, d), c - d);
            return (comb(c, d), c - d);
        }
        else {
            return (0, 0);
        }
    }
    else {
        let mut cprd = 1;
        let mut csum = c;
        for &nnode in g[node].0.iter() {
            let (cc, rc) = sub(g, nnode);
            cprd = (cprd * cc) % 998244353;
            csum += rc;
        }
        if csum >= d {
            cprd = (cprd * comb(csum, d)) % 998244353;
            debug!(node, cprd, csum - d);
            return (cprd, csum - d);
        }
        else {
            return (0, 0);
        }
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let n: usize = io.next();
    let mut g = vec![(BTreeSet::new(), 0, 0); n];
    for i in 1..n {
        let p: usize = io.next();
        let p = p - 1;
        g[p].0.insert(i);
    }
    for i in 0..n {
        let c: usize = io.next();
        g[i].1 = c;
    }
    for i in 0..n {
        let d: usize = io.next();
        g[i].2 = d;
    }
    debug!(g);

    let a = sub(&g, 0);
    io.put(a.0);
    io.putn();
}
