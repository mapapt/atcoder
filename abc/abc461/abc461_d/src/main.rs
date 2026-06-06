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

    let h: usize = io.next();
    let w: usize = io.next();
    let k: u32 = io.next();
    let mut m = Vec::new();
    for _ in 0..h {
        let s = io.next_string(); // String
        let mut v = Vec::new();
        let mut cum = 0;
        v.push(cum);
        for c in s.chars() {
            if c == '1' {
                cum += 1;
            }
            v.push(cum);
        }
        m.push(v);
    }
    debug!(m);

    let mut ans = 0_usize;
    for j0 in 0..w {
        for j1 in j0..w {
            let cnt_gt_k = {
                let mut i0 = 0;
                let mut i1 = 0;
                let cum = m[0][j1 + 1] - m[0][j0];
                let mut sum = cum;
                let mut cnt = 0;
                loop {
                    if sum > k {
                        cnt += h - i1;

                        if i0 < i1 {
                            let cum = m[i0][j1 + 1] - m[i0][j0];
                            i0 += 1;
                            sum -= cum;
                        }
                        else if i0 < h - 1 {
                            i0 += 1;
                            i1 += 1;
                            let cum = m[i0][j1 + 1] - m[i0][j0];
                            sum = cum;
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        if i1 < h - 1 {
                            i1 += 1;
                            let cum = m[i1][j1 + 1] - m[i1][j0];
                            sum += cum;
                        }
                        else {
                            break;
                        }
                    }
                }
                cnt
            };
            let cnt_ge_k = {
                let mut i0 = 0;
                let mut i1 = 0;
                let cum = m[0][j1 + 1] - m[0][j0];
                let mut sum = cum;
                let mut cnt = 0;
                loop {
                    if sum >= k {
                        cnt += h - i1;

                        if i0 < i1 {
                            let cum = m[i0][j1 + 1] - m[i0][j0];
                            i0 += 1;
                            sum -= cum;
                        }
                        else if i0 < h - 1 {
                            i0 += 1;
                            i1 += 1;
                            let cum = m[i0][j1 + 1] - m[i0][j0];
                            sum = cum;
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        if i1 < h - 1 {
                            i1 += 1;
                            let cum = m[i1][j1 + 1] - m[i1][j0];
                            sum += cum;
                        }
                        else {
                            break;
                        }
                    }
                }
                cnt
            };
            ans += cnt_ge_k - cnt_gt_k;
        }
    }
    io.put(ans);
    io.putn();
}
