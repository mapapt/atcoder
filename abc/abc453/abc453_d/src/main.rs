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
    let mut ss = Vec::new();
    let mut start = (0, 0);
    for i in 0..h {
        let s = io.next_string(); // String
        let mut s: Vec<char> = s.chars().collect();
        for j in 0..w {
            if s[j] == 'S' {
                start = (i, j);
                s[j] = '.';
            }
        }
        ss.push(s);
    }
    debug!(ss);
    debug!(start);

    let mut vis = BTreeSet::new();
    let mut q = VecDeque::new();
    q.push_front((start.0, start.1, VecDeque::new(), 'S'));
    while let Some((i, j, mut path, dir)) = q.pop_front() {
        if !vis.contains(&(i, j, dir)) {
            if dir != 'S' {
                path.push_back(dir);
            }
            match ss[i][j] {
                '.' => {
                    vis.insert((i, j, 'U'));
                    vis.insert((i, j, 'L'));
                    vis.insert((i, j, 'D'));
                    vis.insert((i, j, 'R'));
                    if i > 0 {
                        q.push_front((i - 1, j, path.clone(), 'U'));
                    }
                    if j > 0 {
                        q.push_front((i, j - 1, path.clone(), 'L'));
                    }
                    if i < h - 1 {
                        q.push_front((i + 1, j, path.clone(), 'D'));
                    }
                    if j < w - 1 {
                        q.push_front((i, j + 1, path.clone(), 'R'));
                    }
                },
                'o' => {
                    vis.insert((i, j, dir));
                    if dir == 'U' && i > 0 {
                        q.push_front((i - 1, j, path.clone(), 'U'));
                    }
                    if dir == 'L' && j > 0 {
                        q.push_front((i, j - 1, path.clone(), 'L'));
                    }
                    if dir == 'D' && i < h - 1 {
                        q.push_front((i + 1, j, path.clone(), 'D'));
                    }
                    if dir == 'R' && j < w - 1 {
                        q.push_front((i, j + 1, path.clone(), 'R'));
                    }
                },
                'x' => {
                    vis.insert((i, j, dir));
                    if dir != 'U' && i > 0 {
                        q.push_front((i - 1, j, path.clone(), 'U'));
                    }
                    if dir != 'L' && j > 0 {
                        q.push_front((i, j - 1, path.clone(), 'L'));
                    }
                    if dir != 'D' && i < h - 1 {
                        q.push_front((i + 1, j, path.clone(), 'D'));
                    }
                    if dir != 'R' && j < w - 1 {
                        q.push_front((i, j + 1, path.clone(), 'R'));
                    }
                },
                'G' => {
                    io.puty(true);
                    for c in path {
                        print!("{c}");
                    }
                    return;
                },
                _ => {}
            }
        }
    }
    io.puty(false);
}
