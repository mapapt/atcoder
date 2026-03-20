use std::prelude::rust_2021::*;
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

struct Tokens<'a>(std::str::SplitWhitespace<'a>);

#[allow(dead_code)]
impl<'a> Tokens<'a> {
    fn new(buf: &'a mut String) -> Self {
        std::io::stdin().read_to_string(buf).unwrap();
        Tokens(buf.split_whitespace())
    }
    fn release(self) -> std::str::SplitWhitespace<'a> {
        self.0
    }
    fn next_string(&mut self) -> String {
        self.0.next().unwrap().to_string()
    }
    fn next_bytes(&mut self) -> Vec<u8> {
        self.0.next().unwrap().as_bytes().to_vec()
    }
    fn next<T>(&mut self) -> T
    where T: std::str::FromStr, T::Err: std::fmt::Debug {
        self.0.next().unwrap().parse().unwrap()
    }
    fn collect<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<T> {
        (0..n).map(|_| self.next()).collect()
    }
    fn collect_index<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<(usize, T)> {
        (0..n).map(|i| (i, self.next())).collect()
    }
}

//#############################################################################

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let m: usize = tokens.next();

    let mut e = Vec::new();

    for _ in 0..m {
        let k: usize = tokens.next();
        let c: u64 = tokens.next();
        let mut a: Vec<usize> = tokens.collect(k);

        a.sort();

        for ai in a.iter().skip(1) {
            e.push((c, a[0], *ai));
        }
    }

    debug!(e);

    let mut map = BTreeMap::new();
    let mut score = 0;
    let mut cnt = 0;

    for ei in e.iter() {
        let ei1 = map.contains_key(&ei.1);
        let ei2 = map.contains_key(&ei.2);
        if !ei1 && !ei2 {
            map.insert(ei.1, ei.1.min());
            map.insert(ei.2, ei.1);
            score += ei.0;
            cnt += 1;
        }
        else if ei1 {
            map.insert(ei.2, ei.1);
            score += ei.0;
            cnt += 1;
        }
        else if ei2 {
            map.insert(ei.1, ei.2);
            score += ei.0;
            cnt += 1;
        }
        else {
            let mut root1 = ei.1;
            let mut pos = ei.1;
            while let Some(&npos) = map.get(&pos) {
                pos = npos;
                root1 = root1.min(npos);
            }
            let mut root2 = ei.2;
            let mut pos = ei.2;
            while let Some(&npos) = map.get(&pos) {
                pos = npos;
                root2 = root2.min(npos);
            }

            if root1 != root2 {
                map.insert(root2.max(root1), root2.min(root1));
                score += ei.0;
                cnt += 1;
            }
        }
    }

    debug!(map);

    if cnt < n - 1 {
        println!("-1");
    }
    else {
        println!("{score}");
    }
}
