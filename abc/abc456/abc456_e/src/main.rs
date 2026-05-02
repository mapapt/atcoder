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

fn sub(g: &Vec<HashSet<usize>>, cds: &mut BTreeSet<usize>, vis: &mut HashSet<usize>, cd: usize) -> bool {
    cds.remove(&cd);
    if vis.contains(&cd) {
        true
    }
    else {
        vis.insert(cd);
        for &ncd in g[cd].iter() {
            if sub(g, cds, vis, ncd) {
                vis.remove(&cd);
                return true;
            }
        }
        vis.remove(&cd);
        false
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let t: usize = io.next();

    for _ in 0..t {
        let n: usize = io.next();
        let m: usize = io.next();

        let mut g1 = vec![HashSet::new(); n];
        for _ in 0..m {
            let u: usize = io.next();
            let v: usize = io.next();
            let u = u - 1;
            let v = v - 1;

            g1[u].insert(v);
            g1[v].insert(u);
        }
        for c in 0..n {
            g1[c].insert(c);
        }
        debug!(g1);

        let w: usize = io.next();

        let mut g2 = vec![HashSet::new(); n];
        for c in 0..n {
            let s = io.next_string(); // String
            for (d, o) in s.char_indices() {
                if o == 'o' {
                    g2[c].insert(d);
                }
            }
        }
        debug!(g2);

        let mut cds = BTreeSet::new();
        let mut g = vec![HashSet::new(); n * w];
        for c in 0..n {
            for d in g2[c].iter() {
                let nd = (d + 1) % w;

                let cd = c * w + d;
                cds.insert(cd);
                for &nc in g1[c].iter() {
                    if g2[nc].contains(&nd) {
                        let ncd = nc * w + nd;
                        g[cd].insert(ncd);
                    }
                }
            }
        }
        debug!(g);

        let mut ok = false;
        while let Some(&scd) = cds.first() {
            let mut vis = HashSet::new();
            if sub(&g, &mut cds, &mut vis, scd) {
                ok = true;
                break;
            }
        }
        io.puty(ok);
    }
}
