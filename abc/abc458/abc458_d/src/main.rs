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

#[derive(Debug)]
struct SkipNode<T>
{
    value: Option<Rc<T>>,
    width: usize,
    next: Option<usize>,
    down: Option<usize>,
}

struct SkipList<T>
{
    max_level: usize,
    nodes: Vec<SkipNode<T>>,
    free_head: Option<usize>,
    rnd_st: u64,
}

impl<T> SkipList<T>
{
    fn new() -> SkipList<T> {
        let max_level = 32;
        let mut nodes = Vec::new();
        for i in 0..=max_level {
            nodes.push(SkipNode {
                value: None,
                width: 0,
                next: None,
                down: if i < max_level {Some(i + 1)} else {None},
            });
        }
        SkipList {
            max_level: max_level,
            nodes: nodes,
            free_head: None,
            rnd_st: 88172645463325252,
        }
    }
    fn len(&self) -> usize {
        let mut c_id = 0;
        let mut sum_width = 0;
        while let Some(r_id) = self.nodes[c_id].next {
            sum_width += self.nodes[c_id].width;
            c_id = r_id;
        }
        sum_width += self.nodes[c_id].width;
        sum_width
    }
}

impl<T: Ord> SkipList<T>
{
    fn h_xorshift64(state: u64) -> u64 {
        let state = state ^ (state << 13);
        let state = state ^ (state >> 7);
        let state = state ^ (state << 17);
        state
    }
    fn h_insert(&mut self, index: usize, value: Rc<T>, level: usize) -> (Option<usize>, usize) {
        //println!("{} {:?} {}", index, self.nodes[&index].value, level);
        let mut c_id = index;
        let mut sum_width = 0;
        while let Some(r_id) = self.nodes[c_id].next {
            let right = &self.nodes[r_id];
            if &value >= right.value.as_ref().unwrap() {
                sum_width += self.nodes[c_id].width;
                c_id = r_id;
            }
            else {
                break;
            }
        }

        let (a_id, a_width) = if let Some(b_id) = self.nodes[c_id].down {
            self.h_insert(b_id, value.clone(), level.max(1) - 1)
        }
        else {
            (None, 1)
        };

        if level == 0 {
            let n_node = SkipNode {
                value: Some(value),
                width: self.nodes[c_id].width + 1 - a_width,
                next: self.nodes[c_id].next,
                down: a_id,
            };

            let n_id = if let Some(f_idx) = self.free_head {
                self.free_head = self.nodes[f_idx].next;
                self.nodes[f_idx] = n_node;
                f_idx
            }
            else {
                self.nodes.push(n_node);
                self.nodes.len() - 1
            };

            self.nodes[c_id].next = Some(n_id);
            self.nodes[c_id].width = a_width;

            //println!("---");
            //println!("{}: {:?}", c_id, cur_mut);
            //println!("{}: {:?}", self.max_index, self.nodes[&self.max_index]);
            //println!("{} {}", sum_width, a_width);

            (Some(n_id), sum_width + a_width)
        }
        else {
            self.nodes[c_id].width += 1;

            //println!("{:?}", cur_mut);
            //println!("extend {}", sum_width + a_width);

            (None, sum_width + a_width)
        }
    }
    //
    fn insert(&mut self, value: T) -> usize {
        self.rnd_st = Self::h_xorshift64(self.rnd_st);

        let level = self.rnd_st.trailing_zeros() as usize;
        let level = level.min(self.max_level - 1);

        //println!("{:?} {}", value, level);
        let (_, width) = self.h_insert(0, Rc::new(value), self.max_level - level);
        width - 1
    }
}

impl<T: Ord> SkipList<T>
{
    fn search(&self, x: &T) -> Result<usize, usize> {
        let mut c_id = 0;
        let mut sum_width = 0;
        loop {
            //println!("{}", c_id);
            if let Some(r_id) = self.nodes[c_id].next {
                let right = &self.nodes[r_id];
                if x >= right.value.as_ref().unwrap() {
                    sum_width += self.nodes[c_id].width;
                    c_id = r_id;
                    continue;
                }
            }

            if let Some(b_id) = self.nodes[c_id].down {
                c_id = b_id;
                continue;
            }
            else {
                break;
            }
        }

        //println!("{}", c_id);
        if let Some(y) = &self.nodes[c_id].value {
            if x == y.as_ref() {
                return Ok(sum_width - 1);
            }
        }
        Err(sum_width)
    }
}

impl<T: Ord> SkipList<T>
{
    fn remove(&mut self, index: usize) -> Option<T> {
        let mut value = None;

        let mut c_id = 0;
        let mut sum_width = 0;
        loop {
            if index + 1 > sum_width + self.nodes[c_id].width {
                if let Some(r_id) = self.nodes[c_id].next {
                    sum_width += self.nodes[c_id].width;
                    c_id = r_id;
                    continue;
                }
                else {
                    return None;
                }
            }
            else if index + 1 < sum_width + self.nodes[c_id].width {
                self.nodes[c_id].width -= 1;

                if let Some(b_id) = self.nodes[c_id].down {
                    c_id = b_id;
                    continue;
                }
                else {
                    break;
                }
            }
            else {
                self.nodes[c_id].width -= 1;

                if let Some(r_id) = self.nodes[c_id].next {
                    self.nodes[c_id].next = self.nodes[r_id].next;
                    self.nodes[c_id].width += self.nodes[r_id].width;
                    value = self.nodes[r_id].value.take();

                    self.nodes[r_id].next = self.free_head;
                    self.free_head = Some(r_id);
                }

                if let Some(b_id) = self.nodes[c_id].down {
                    c_id = b_id;
                    continue;
                }
                else {
                    break;
                }
            }
        }

        Rc::into_inner(value.unwrap())
    }
}

impl<T> Index<usize> for SkipList<T>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut c_id = 0;
        let mut sum_width = 0;
        loop {
            //println!("{}", c_id);
            if index + 1 == sum_width {
                return self.nodes[c_id].value.as_ref().unwrap();
            }
            else {
                if index + 1 >= sum_width + self.nodes[c_id].width {
                    if let Some(r_id) = self.nodes[c_id].next {
                        sum_width += self.nodes[c_id].width;
                        c_id = r_id;
                        continue;
                    }
                }
                if let Some(b_id) = self.nodes[c_id].down {
                    c_id = b_id;
                    continue;
                }
                panic!("Index:{} not found", index);
            }
        }
    }
}

struct SkipListIter<'a, T>
{
    nodes: &'a[SkipNode<T>],
    c_id: usize,
}

impl<'a, T> Iterator for SkipListIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(n_id) = self.nodes[self.c_id].next {
            self.c_id = n_id;
            Some(self.nodes[self.c_id].value.as_ref().unwrap())
        }
        else {
            None
        }
    }
}

impl<T> SkipList<T>
{
    fn iter(&self) -> SkipListIter<'_, T> {
        SkipListIter {
            nodes: &self.nodes,
            c_id: self.max_level,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SkipList<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let x: u32 = io.next();
    let q: usize = io.next();

    let mut s = SkipList::new();
    s.insert(x);
    for i in 0..q {
        let a: u32 = io.next();
        let b: u32 = io.next();
        s.insert(a);
        s.insert(b);

        io.put(s[i + 1]);
        io.putn();
    }
}
