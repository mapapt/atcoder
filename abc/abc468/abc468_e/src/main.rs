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
impl Int for i8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for isize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }

// (g, x, y) s.t. a x + b y = gcd(a, b) = g
fn ext_euclid<N: Int>(a: N, b: N) -> (N, N, N)
{
    let zero = N::default();
    let one = b / b;

    let mut r0 = a;
    let mut s0 = one;
    let mut t0 = zero;
    let mut r1 = b;
    let mut s1 = zero;
    let mut t1 = one;

    loop {
        let q1 = r0 / r1;
        let r2 = r0 - q1 * r1;

        if r2 == zero {
            return (r1, s1, t1);
        }

        let s2 = s0 - q1 * s1;
        let t2 = t0 - q1 * t1;

        (r0, r1) = (r1, r2);
        (s0, s1) = (s1, s2);
        (t0, t1) = (t1, t2);
    }
}

fn mod_div<N: Int>(x: N, y: N, m: N) -> N
{
    let zero = N::default();
    let one = m / m;

    let (gcd, y_inv, _) = ext_euclid(y, m);

    if gcd != one {panic!();}

    let y_inv = if y_inv < zero {y_inv + m} else {y_inv};

    (x * y_inv) % m
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let n: usize = io.next();
    let a: Vec<i64> = io.collect(n);
    let m = 998244353;

    let sa: i64 = a.iter().sum();
    let sa = sa % m;
    debug!(sa);
    let mut i2a = Vec::new();
    {
        let mut ia = Vec::new();
        let mut x = 0;
        for ai in &a {
            x = (x + ai) % m;
            ia.push(x);
        }
        let mut x2 = 0;
        i2a.push(x2);
        for iai in &ia {
            x2 = (x2 + iai) % m;
            i2a.push(x2);
        }
    }
    let i2a = i2a;
    debug!(i2a);
    let mut i2ar = Vec::new();
    {
        let mut ia = Vec::new();
        let mut x = 0;
        for ai in a.iter().rev() {
            x = (x + ai) % m;
            ia.push(x);
        }
        let mut x2 = 0;
        i2ar.push(x2);
        for iai in &ia {
            x2 = (x2 + iai) % m;
            i2ar.push(x2);
        }
    }
    let i2ar = i2ar;
    debug!(i2ar);

    let mut ans = 0;
    for len in 1..=n {
        let den = len as i64;
        //let num = sa * den - i2a[len - 1] - i2ar[len - 1];
        let num = (sa * den) % m;
        let num = if num >= i2a[len - 1] {
            num - i2a[len - 1]
        }
        else {
            num + (m - i2a[len - 1])
        };
        let num = if num >= i2ar[len - 1] {
            num - i2ar[len - 1]
        }
        else {
            num + (m - i2ar[len - 1])
        };
        //debug!(num, den);

        let r = mod_div(num, den, m);
        //debug!(r);

        ans = (ans + r) % m;
    }
    io.put(ans);
    io.putn();
}
