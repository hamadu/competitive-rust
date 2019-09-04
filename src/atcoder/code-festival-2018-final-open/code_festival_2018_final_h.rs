// https://atcoder.jp/contests/code-festival-2018-final-open/tasks/code_festival_2018_final_h
//
#![allow(unused_imports)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::rc::*;
use std::cell::*;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

type V = usize;
type Graph = Vec<Vec<V>>;

#[derive(Debug)]
struct Lca {
    tree: Graph,
    parents: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl Lca {
    fn new(tree: Graph) -> Self {
        let n = tree.len();
        let pn = (64 - (n as u64).leading_zeros()) as usize;

        let mut parents = vec![vec![0; n]; pn];
        let mut depth = vec![0; n];

        Lca::dfs(&tree, &mut parents, &mut depth, 0, n);

        for i in 1..pn {
            for j in 0..n {
                parents[i][j] = parents[i-1][parents[i-1][j]];
            }
        }

        Lca {
            tree: tree,
            parents: parents,
            depth: depth,
        }
    }

    fn dfs(tree: &Graph, parents: &mut Vec<Vec<usize>>, depth: &mut Vec<usize>, now: usize, par: usize) {
        if par != tree.len() {
            parents[0][now] = par;
            depth[now] = depth[par] + 1;
        }
        for &to in &tree[now] {
            if par == to {
                continue;
            }
            Lca::dfs(tree, parents, depth, to, now);
        }
    }

    fn lca(&self, u: usize, v: usize) -> usize {
        if u == v {
            return u;
        } else if self.depth[u] > self.depth[v] {
            return self.lca(v, u);
        } else if self.depth[u] < self.depth[v] {
            let mut diff = self.depth[v] - self.depth[u];
            let mut v = v;
            let mut pi = 0;
            while diff >= 1 {
                if diff & 1 == 1 {
                    v = self.parents[pi][v];
                }
                pi += 1;
                diff >>= 1;
            }
            return self.lca(u, v);
        } else {
            let mut pi = self.parents.len();
            let mut u = u;
            let mut v = v;
            while pi > 0 {
                pi -= 1;
                if self.parents[pi][u] != self.parents[pi][v] {
                    u = self.parents[pi][u];
                    v = self.parents[pi][v];
                }
            }
            assert!(self.parents[0][u] == self.parents[0][v]);
            self.parents[0][u]
        }
    }
}

// ===

#[derive(Debug)]
struct Centeroid {
    tree: Graph,
    root: usize,
    parent: Vec<usize>
}

impl Centeroid {
    fn new(tree: Graph) -> Self {
        let n = tree.len();
        let mut parent = vec![n; n];
        let root = Self::decompose(0, None, &tree, &mut vec![false; n], &mut vec![0; n], &mut parent);
        Centeroid { tree: tree, root: root, parent: parent }
    }

    fn decompose(root: usize, p: Option<usize>, tree: &Graph, deleted: &mut Vec<bool>, count: &mut Vec<usize>, parent: &mut Vec<usize>) -> usize {
        let total = Self::count_children(root, p, tree, deleted, count);
        let center = Self::find_centeroid(root, None, tree, deleted, count, total).unwrap();
        parent[center] = p.unwrap_or(tree.len());

        deleted[center] = true;
        for &to in &tree[center] {
            if deleted[to] {
                continue;
            }
            Self::decompose(to, Some(center), tree, deleted, count, parent);
        }
        deleted[center] = false;
        center
    }

    fn find_centeroid(now: usize, p: Option<usize>, tree: &Graph, deleted: &Vec<bool>, count: &Vec<usize>, total: usize) -> Option<usize> {
        let mut center = tree.len();
        let mut max_children = total - count[now];
        for &to in &tree[now] {
            if Some(to) == p || deleted[to] {
                continue;
            }
            if let Some(s) = Self::find_centeroid(to, Some(now), tree, deleted, count, total) {
                return Some(s);
            }
            max_children = max(max_children, count[to]);
        }
        if max_children > total/2 {
            return None;
        }
        Some(now)
    }

    fn count_children(now: usize, p: Option<usize>, tree: &Graph, deleted: &Vec<bool>, count: &mut Vec<usize>) -> usize {
        let mut cnt = 1;
        for &to in &tree[now] {
            if Some(to) == p || deleted[to] {
                continue;
            }
            cnt += Self::count_children(to, Some(now), tree, deleted, count);
        }
        count[now] = cnt;
        cnt
    }
}

// ====

#[derive(Debug, Clone)]
struct Treap<T> {
    total: usize,
    root: Link<T>
}

impl<T: Ord + Copy + Debug> Treap<T> {
    fn new() -> Self {
        Treap {
            total: 0,
            root: None
        }
    }

    fn push(&mut self, v: T) -> bool {
        if self.contains(v) {
            return false;
        }
        let u = self.root.take();
        self.root = Node::push(u, v);
        self.total += 1;
        true
    }

    fn remove(&mut self, v: T) -> bool {
        let r = Node::remove(&mut self.root, v);
        if r { self.total -= 1; }
        r
    }

    fn len(&self) -> usize {
        self.total
    }

    fn min(&self) -> Option<T> {
        Node::min(&self.root)
    }

    fn max(&self) -> Option<T> {
        Node::max(&self.root)
    }

    fn contains(&self, value: T) -> bool {
        Node::contains(&self.root, value)
    }

    fn peek_less(&self, value: T) -> Option<T> {
        Node::peek_less(&self.root, value)
    }

    fn peek_greater(&self, value: T) -> Option<T> {
        Node::peek_greater(&self.root, value)
    }
}

struct XorShift {
    x: i64,
    y: i64,
    z: i64,
    w: i64
}

impl XorShift {
    fn rotate(&mut self) {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
    }

    fn next_i64(&mut self) -> i64 {
        self.rotate();
        self.w
    }
}

static mut RAND: XorShift = XorShift {
    x: 123456789i64,
    y: 362436069i64,
    z: 521288629i64,
    w: 88675123i64
};

fn generate_next() -> i64 {
    let x;
    unsafe {
        x = RAND.next_i64();
    }
    x
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    size: usize,
    priority: i64,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Copy + Debug> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            size: 1,
            priority: generate_next(),
            left: None,
            right: None
        }
    }

    fn sz(node: &Link<T>) -> usize {
        match node.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => 0,
            Some(x) => x.size
        }
    }

    fn merge(left: Link<T>, right: Link<T>) -> Link<T> {
        match (left, right) {
            (None, None) => None,
            (None, x) => x,
            (x, None) => x,
            (Some(l), Some(r)) => {
                if l.priority < r.priority {
                    let mut l = l;
                    l.right = Self::merge(l.right.take(), Some(r));
                    Some(l)
                } else {
                    let mut r = r;
                    r.left = Self::merge(Some(l), r.left.take());
                    Some(r)
                }
            }
        }
    }

    fn min(now: &Link<T>) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None    => None,
            Some(x) => {
                match x.left.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
                    None    => Some(x.value),
                    Some(_) => Self::min(&x.left)
                }
            }
        }
    }

    fn max(now: &Link<T>) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None    => None,
            Some(x) => {
                match x.right.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
                    None    => Some(x.value),
                    Some(_) => Self::max(&x.right)
                }
            }
        }
    }

    fn contains(now: &Link<T>, value: T) -> bool {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => false,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Less    => Self::contains(&x.left, value),
                    Ordering::Equal   => true,
                    Ordering::Greater => Self::contains(&x.right, value)
                }
            }
        }
    }

    fn peek_less(now: &Link<T>, value: T) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => None,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Less | Ordering::Equal => {
                        Self::peek_less(&x.left, value)
                    },
                    Ordering::Greater => {
                        let w = Self::peek_less(&x.right, value);
                        if w == None {
                            Some(x.value)
                        } else {
                            Some(max(w.unwrap(), x.value))
                        }
                    }
                }
            }
        }
    }

    fn peek_greater(now: &Link<T>, value: T) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => None,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Greater | Ordering::Equal => {
                        Self::peek_greater(&x.right, value)
                    },
                    Ordering::Less => {
                        let w = Self::peek_greater(&x.left, value);
                        if w == None {
                            Some(x.value)
                        } else {
                            Some(min(w.unwrap(), x.value))
                        }
                    }
                }
            }
        }
    }

    fn split_at(now: Link<T>, k: usize) -> (Link<T>, Link<T>) {
        match now {
            None => (None, None),
            Some(x) => {
                let mut x = x;
                let lsz = Self::sz(&x.left);
                if k <= lsz {
                    let (nl, nr) = Self::split_at(x.left.take(), k);
                    x.left = nr;
                    (nl, Some(x))
                } else {
                    let (nl, nr) = Self::split_at(x.right.take(), k-1-lsz);
                    x.right = nl;
                    (Some(x), nr)
                }
            },
        }
    }

    fn split_by(now: Link<T>, v: T) -> (Link<T>, Link<T>) {
        match now {
            None => (None, None),
            Some(x) => {
                let mut x = x;
                if v <= x.value {
                    let (nl, nr) = Self::split_by(x.left.take(), v);
                    x.left = nr;
                    (nl, Some(x))
                } else {
                    let (nl, nr) = Self::split_by(x.right.take(), v);
                    x.right = nl;
                    (Some(x), nr)
                }
            },
        }
    }

    fn push(now: Link<T>, new_value: T) -> Link<T> {
        let new_node = Some(Box::new(Node::new(new_value)));
        let (left, right) = Self::split_by(now, new_value);
        Self::merge(Self::merge(left, new_node), right)
    }

    fn remove(now: &mut Link<T>, target_value: T) -> bool {
        let mut both_none = false;
        let w = match now.as_mut() { // TODO: Remove as_mut() after upgrading above 1.26
            None => false,
            Some(x) => {
                if target_value < x.value {
                    Self::remove(&mut x.left, target_value)
                } else if x.value < target_value {
                    Self::remove(&mut x.right, target_value)
                } else {
                    let new_link = Self::merge(x.left.take(), x.right.take());
                    if let Some(node) = new_link {
                        *x = node;
                    } else {
                        both_none = true;
                    }
                    true
                }
            }
        };
        if both_none {
            now.take();
        }
        w
    }
}

// ====

const INF: i64 = 1e18 as i64;

fn get_time_value(t: &Treap<(i64, i64)>, time: i64) -> i64 {
    if let Some(w) = t.peek_less((time, INF)) {
        w.1
    } else {
        0
    }
}

fn update_time_value(t: &mut Treap<(i64, i64)>, time: i64, value: i64) {
    if let Some(w) = t.peek_greater((time, value)) {
        if w.0 == time {
            assert!(value < w.1);
            return;
        }
    }
    if let Some(w) = t.peek_less((time-1, INF)) {
        if w.1 >= value {
            return;
        }
    }
    while let Some(w) = t.peek_greater((time, value)) {
        if w.1 <= value {
            t.remove(w);
        } else {
            break;
        }
    }
    t.push((time, value));
}

fn dist(a: usize, b: usize, lca: &Lca, depth: &Vec<i64>) -> i64 {
    let w = lca.lca(a, b);
    depth[a] + depth[b] - 2 * depth[w]
}

fn dfsdep(now: usize, par: usize, de: i64, graph: &Vec<Vec<(usize, i64)>>, depth: &mut Vec<i64>) {
    depth[now] = de;
    for &(to, len) in &graph[now] {
        if to == par {
            continue;
        }
        dfsdep(to, now, de+len, graph, depth);
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(usize1, usize1, i64); n-1],
        queries: [(i64, i64, usize1, i64); m]
    };


    let mut tree = vec![vec![]; n];
    let mut treew = vec![vec![]; n];
    for &e in &edges {
        tree[e.0].push(e.1);
        tree[e.1].push(e.0);

        treew[e.0].push((e.1, e.2));
        treew[e.1].push((e.0, e.2));
    }
    let comp = Centeroid::new(tree.clone());
    let center = comp.root;
    let lca = Lca::new(tree.clone());
    let mut depth = vec![0; n];
    dfsdep(0, n, 0, &treew, &mut depth);

    let mut tps = vec![Treap::new(); n];
    for i in 0..n {
        tps[i].push((0, 0));
    }
    let mut ans = 0;
    let mut queries = queries;
    queries.sort();
    for (from, to, at, value) in queries {
        let mut wat = at;
        let mut best = 0;
        loop {
            let before_on = from - dist(at, wat, &lca, &depth);
            best = max(best, get_time_value(&tps[wat], before_on));
            if wat == center {
                break;
            }
            wat = comp.parent[wat];
        }
        let score = best + value;
        ans = max(ans, score);
        let mut wat = at;
        loop {
            let after_on = to + dist(at, wat, &lca, &depth);
            update_time_value(&mut tps[wat], after_on, score);
            if wat == center {
                break;
            }
            wat = comp.parent[wat];
        }
    }
    println!("{}", ans);
}
