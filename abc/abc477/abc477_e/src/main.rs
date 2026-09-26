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

#[derive(Debug, Clone)]
struct Edge<W> {
    weight: W,
    from: usize,
    to: usize,
}

impl<W: Copy> Edge<W> {
    fn node_from(&self, u: usize) -> (usize, W) {
        let nu = if u != self.to {
            self.to
        }
        else {
            self.from
        };

        (nu, self.weight)
    }
}

#[derive(Debug, Clone)]
enum NodeSt {
    Unvisited,
    Returned,
    Visited,
}

#[derive(Debug, Clone)]
struct Graph<V, W> {
    node_values: Vec<V>,
    node_edges: Vec<BTreeSet<usize>>,
    edges: Vec<Edge<W>>,
    undir: bool,
}

impl<V, W: Copy> Graph<V, W> {
    fn new(undir: bool) -> Self {
        Graph {
            node_values: Vec::new(),
            node_edges: Vec::new(),
            edges: Vec::new(),
            undir,
        }
    }

    fn new_nodes(n: usize, value: V, undir: bool) -> Self
    where V: Clone {
        Graph {
            node_values: vec![value; n],
            node_edges: vec![BTreeSet::new(); n],
            edges: Vec::new(),
            undir,
        }
    }

    fn add_node(&mut self, value: V) {
        self.node_values.push(value);
        self.node_edges.push(BTreeSet::new());
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        let edge = Edge {
            weight,
            from,
            to,
        };

        let edge_idx = self.edges.len();

        self.edges.push(edge);

        self.node_edges[from].insert(edge_idx);
        if self.undir {
            self.node_edges[to].insert(edge_idx);
        }
    }

    fn node_values(&self) -> &[V] {
        &self.node_values
    }

    fn _traverse<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        mut unvis: BTreeSet<usize>,
        bfs: bool,
        mut func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        let mut que = VecDeque::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push_front((first_node, first_weight, first_travel));

        while let Some((u, w, t)) = que.pop_front() {
            if unvis.contains(&u) {
                unvis.remove(&u);

                let nt = func(NodeSt::Unvisited, &mut self.node_values[u], w, t);

                for &e in self.node_edges[u].iter() {
                    let (nu, nw) = self.edges[e].node_from(u);

                    if bfs {
                        que.push_back((nu, nw, nt));
                    }
                    else {
                        que.push_front((nu, nw, nt));
                    }
                }
            }
            else {
                func(NodeSt::Visited, &mut self.node_values[u], w, t);
            }
        }

        unvis
    }

    fn dfs<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvis: BTreeSet<usize>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvis, false, func)
    }
    
    fn bfs<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvis: BTreeSet<usize>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvis, true, func)
    }
    
    fn dijkstra<U>(&mut self,
        first_node: Option<usize>,
        mut unvis: BTreeSet<usize>,
        mut update: U) -> BTreeSet<usize>
    where U: FnMut(&mut V, W, usize) -> bool, W: Ord + Add<Output=W> + Default {

        let mut que = BinaryHeap::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push((Reverse(W::default()), first_node, first_node));

        while let Some((ws, u, prev_u)) = que.pop() {
            unvis.remove(&u);

            if update(&mut self.node_values[u], ws.0, prev_u) {

                for &e in self.node_edges[u].iter() {
                    let (nu, nw) = self.edges[e].node_from(u);
                    let nws = ws.0 + nw;

                    que.push((Reverse(nws), nu, u));
                }
            }
        }

        unvis
    }
    
    fn _dfs_rec<T, F>(
        node_values: &mut[V], node_edges: &[BTreeSet<usize>], edges: &[Edge<W>],
        u: usize, w: W, t: T,
        unvis: &mut BTreeSet<usize>,
        func: &mut F)
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        if unvis.contains(&u) {
            unvis.remove(&u);

            let nt = func(NodeSt::Unvisited, &mut node_values[u], w, t);

            for &e in node_edges[u].iter() {
                let (nu, nw) = edges[e].node_from(u);

                Self::_dfs_rec(node_values, node_edges, edges, nu, nw, nt, unvis, func);
            }

            func(NodeSt::Returned, &mut node_values[u], w, t);
        }
        else {
            func(NodeSt::Visited, &mut node_values[u], w, t);
        }
    }

    fn dfs_rec<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        mut unvis: BTreeSet<usize>,
        mut func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        Self::_dfs_rec(
            &mut self.node_values, &self.node_edges, &self.edges,
            first_node, first_weight, first_travel,
            &mut unvis,
            &mut func);

        unvis
    }
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut io = StdIo::new(&mut placeholder);

    let n: usize = io.next();
    let q: usize = io.next();
    let a: Vec<u64> = io.collect(n);
    let b: Vec<u64> = io.collect(n);

    let mut sa = Vec::new();
    let mut sum = 0;
    sa.push(sum);
    let mut g = Graph::new_nodes(n + 1, u64::MAX, true);
    for i in 0..n {
        sum += a[i];
        sa.push(sum);
        let j = i + 1;
        g.add_edge(j, (j % n) + 1, a[i]);
    }
    for i in 0..n {
        let j = i + 1;
        g.add_edge(j, 0, b[i]);
    }
    g.dijkstra(Some(0), (0..n).collect(),
        |v, ws, _p| {
            if *v > ws {
                *v = ws;
                true
            }
            else {
                false
            }
        });
    let dg = g.node_values();

    debug!(sa);
    debug!(dg);

    for _ in 0..q {
        let s: usize = io.next();
        let t: usize = io.next();
        let s = if s == n + 1 {0} else {s};
        let t = if t == n + 1 {0} else {t};
        if t == 0 {
            io.put(dg[s]);
            io.putn();
        }
        else {
            let d1 = sa[t - 1] - sa[s - 1];
            let d2 = sa[n] - d1;
            let d3 = dg[s] + dg[t];
            debug!(d1, d2, d3);
            let dm = d1.min(d2).min(d3);
            io.put(dm);
            io.putn();
        }
    }
}
