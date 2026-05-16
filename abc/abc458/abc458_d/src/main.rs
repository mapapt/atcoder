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

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let x: u64 = io.next();
    let q: usize = io.next();
    let ab: Vec<(u64, u64)> = (0..q).map(|_| (io.next(), io.next())).collect();
    debug!(ab);

    let mut v = Vec::new();
    v.push((x, 0));
    for i in 0..q {
        v.push((ab[i].0, i + 1));
        v.push((ab[i].1, i + 1));
    }
    v.sort();
    debug!(v);

    let mut m = HashMap::new();
    for j in 0..v.len() {
        m.insert(v[j], j);
    }
    debug!(m);

    let mut s = BTreeSet::new();
    let mut k = m[&(x, 0)];
    s.insert(k);
    for i in 0..q {
        let ka = m[&(ab[i].0, i + 1)];
        let kb = m[&(ab[i].1, i + 1)];
        s.insert(ka);
        s.insert(kb);
        debug!(s);
        let (ka, kb) = if ka < kb { (ka, kb) } else { (kb, ka) };
        debug!(k, ka, kb);
        if k < ka {
            let kk = s.range(k + 1..).next().unwrap();
            debug!(kk);
            k = *kk;
        }
        else if kb < k {
            let kk = s.range(..=k - 1).last().unwrap();
            debug!(kk);
            k = *kk;
        }
        io.put(v[k].0);
        io.putn();
    }
}
