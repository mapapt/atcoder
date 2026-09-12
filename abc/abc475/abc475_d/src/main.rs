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

trait Int: std::fmt::Debug + Copy + Default + Ord + Eq + ShrAssign + SubAssign +
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + BitAnd<Output=Self> +
    TryInto<usize> + TryFrom<usize>
{ fn chk_mul(self, rhs: Self) -> Option<Self>; }
impl Int for u8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for usize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for isize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }

fn isqrt<N: Int>(n: N) -> N
{
    let zero = N::default();
    if n == zero {
        zero
    }
    else {
        let one = n / n;
        let two = one + one;
        let mut l = one;
        let mut r = n;
        while l + one < r {
            let c = (l + r) / two;
            if let Some(cc) = c.chk_mul(c) {
                if cc > n {
                    r = c;
                }
                else {
                    l = c;
                }
            }
            else {
                l = c;
            }
        }

        l
    }
}

fn prime<V>(n: usize) -> V
where V: Extend<usize> + Default
{
    let n_isqrt = isqrt(n);

    let mut pf = vec![true; n - 1]; // 0..n-1 == 0..=n-2 == 2..=n
    for p in 2..=n_isqrt {
        if pf[p - 2] {
            for k in 2.. {
                let j = p * k;
                if j <= n {
                    pf[j - 2] = false;
                }
                else {
                    break;
                }
            }
        }
    }

    let mut ps = V::default();
    for (i, f) in pf.iter().enumerate() {
        let p = i + 2;
        if *f {
            ps.extend([p.try_into().unwrap_or_default()]);
        }
    }
    ps
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let s = io.next_string(); // String
    let s: Vec<char> = s.chars().collect();

    let n0 = 10_usize.pow((s.len() - 1) as u32);
    let n1 = 10_usize.pow(s.len() as u32);
    debug!(n0, n1);

    let mut ps: VecDeque<usize> = prime(n1);
    while *ps.front().unwrap() < n0 {
        ps.pop_front();
    }
    //debug!(ps);

    for p in ps {
        debug!(p);
        let t = format!("{}", p);
        let t: Vec<char> = t.chars().collect();
        let mut ok = true;
        for i in 0..s.len() {
            for j in (i + 1)..s.len() {
                if (s[i] == s[j]) != (t[i] == t[j]) {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            io.put(p);
            io.putn();
            return;
        }
    }

    io.put(-1);
    io.putn();
    return;
}
