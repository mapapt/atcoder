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

fn sub(v: &mut HashSet<(usize, usize)>, h: usize, w:usize, a:usize, b:usize, i:usize, j: usize, path: String) -> Option<String>
{
    if !v.contains(&(i, j)) && (i, j) != (a, b) {
        v.insert((i, j));
        if (i, j) == (h - 1, w - 1) && v.len() == h * w - 1 {
            return Some(path);
        }
        if i > 0 {
            if let Some(mut path) = sub(v, h, w, a, b, i - 1, j, path.clone()) {
                path.push('U');
                v.remove(&(i, j));
                return Some(path);
            }
        }
        if j > 0 {
            if let Some(mut path) = sub(v, h, w, a, b, i, j - 1, path.clone()) {
                path.push('L');
                v.remove(&(i, j));
                return Some(path);
            }
        }
        if i < h - 1 {
            if let Some(mut path) = sub(v, h, w, a, b, i + 1, j, path.clone()) {
                path.push('D');
                v.remove(&(i, j));
                return Some(path);
            }
        }
        if j < w - 1 {
            if let Some(mut path) = sub(v, h, w, a, b, i, j + 1, path.clone()) {
                path.push('R');
                v.remove(&(i, j));
                return Some(path);
            }
        }
        v.remove(&(i, j));
    }
    None
}

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let t: usize = io.next();
    for _ in 0..t {
        let n: usize = io.next();
        let a: usize = io.next();
        let b: usize = io.next();

        let mut h = n;
        let mut w = n;
        let mut a = a - 1;
        let mut b = b - 1;
        let cu = if a >= 3 {
            let c = (a - 1) / 2;
            let c = c * 2;
            a -= c;
            h -= c;
            (c, w)
        }
        else {
            (0, 0)
        };
        let cl = if b >= 3 {
            let c = (b - 1) / 2;
            let c = c * 2;
            b -= c;
            w -= c;
            (h, c)
        }
        else {
            (0, 0)
        };
        let cd = if a + 3 <= h - 1 {
            let c = (h - 1 - a - 1) / 2;
            let c = c * 2;
            h -= c;
            (c, w)
        }
        else {
            (0, 0)
        };
        let cr = if b + 3 <= w - 1 {
            let c = (w - 1 - b - 1) / 2;
            let c = c * 2;
            w -= c;
            (h, c)
        }
        else {
            (0, 0)
        };
        debug!(h, w, a, b, cu, cl, cd, cr);

        let mut v = HashSet::new();
        if let Some(path) = sub(&mut v, h, w, a, b, 0, 0, String::new()) {
            io.puty(true);
            for _ in 0..cu.0 / 2 {
                for _ in 0..cu.1 - 1 {
                    print!("R");
                }
                print!("D");
                for _ in 0..cu.1 - 1 {
                    print!("L");
                }
                print!("D");
            }
            for _ in 0..cl.1 / 2 {
                for _ in 0..cl.0 - 1 {
                    print!("D");
                }
                print!("R");
                for _ in 0..cl.0 - 1 {
                    print!("U");
                }
                print!("R");
            }
            for c in path.chars().rev() {
                print!("{c}");
            }
            for _ in 0..cd.0 / 2 {
                print!("D");
                for _ in 0..cd.1 - 1 {
                    print!("L");
                }
                print!("D");
                for _ in 0..cd.1 - 1 {
                    print!("R");
                }
            }
            for _ in 0..cr.1 / 2 {
                print!("R");
                for _ in 0..cr.0 - 1 {
                    print!("U");
                }
                print!("R");
                for _ in 0..cr.0 - 1 {
                    print!("D");
                }
            }
            io.putn();
        }
        else{
            io.puty(false);
        }
    }
}
