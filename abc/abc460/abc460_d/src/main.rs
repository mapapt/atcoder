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

    let mut g0 = vec![vec![false; w]; h];
    for i in 0..h {
        let s = io.next_string(); // String
        for (j, c) in s.char_indices() {
            match c {
                '#' => {
                    g0[i][j] = true;
                },
                _ => {}
            }
        }
    }

    let mut q = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if !g0[i][j] {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && nj >= 0 && ni < h as isize && nj < w as isize && g0[ni as usize][nj as usize] {
                            q.push_back((i, j, 0));
                        }
                    }
                }
            }
        }
    }

    let mut g1 = vec![vec![usize::MAX; w]; h];
    while let Some((i, j, c)) = q.pop_front() {
        if g1[i][j] > c {
            g1[i][j] = c;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0 && nj >= 0 && ni < h as isize && nj < w as isize {
                        q.push_back((ni as usize, nj as usize, c + 1));
                    }
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", if g1[i][j] == usize::MAX || g1[i][j] % 2 == 0 {'.'} else {'#'});
        }
        println!();
    }
}
