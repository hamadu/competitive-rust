// https://atcoder.jp/contests/abc133/tasks/abc133_f
//
#![allow(unused_imports)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

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
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

type V = usize;
type Color = usize;
type Graph = Vec<Vec<V>>;
type GraphWithColorAndLength = Vec<Vec<(V, Color, i64)>>;

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

fn dfs(tree: &GraphWithColorAndLength, qonv: &Vec<Vec<(usize, Color, i64, i64)>>, answer: &mut Vec<i64>, color_count: &mut Vec<i64>, color_length: &mut Vec<i64>, now: usize, par: usize, depth: i64) {
    for &(qid, cid, newlength, mult) in &qonv[now] {
        answer[qid] += mult * (depth - color_length[cid] + newlength * color_count[cid]);
    }

    for &(to, color, length) in &tree[now] {
        if to == par {
            continue;
        }
        color_count[color] += 1;
        color_length[color] += length;

        dfs(tree, qonv, answer, color_count, color_length, to, now, depth + length);

        color_count[color] -= 1;
        color_length[color] -= length;
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        edges: [(usize1, usize1, usize1, i64); n-1],
        queries: [(usize1, i64, usize1, usize1); q]
    };

    let mut tree = vec![vec![]; n];
    for &e in &edges {
        tree[e.0].push(e.1);
        tree[e.1].push(e.0);
    }
    let lca = Lca::new(tree);
    let mut qonv = vec![vec![]; n];

    for qid in 0..q {
        let q = queries[qid];
        let (u, v) = (q.2, q.3);
        let p = lca.lca(u, v);
        qonv[u].push((qid, q.0, q.1, 1));
        qonv[v].push((qid, q.0, q.1, 1));
        qonv[p].push((qid, q.0, q.1, -2));
    }

    let mut tree = vec![vec![]; n];
    for &e in &edges {
        tree[e.0].push((e.1, e.2, e.3));
        tree[e.1].push((e.0, e.2, e.3));
    }

    let mut answer = vec![0; q];
    dfs(&mut tree, &qonv, &mut answer, &mut vec![0; n], &mut vec![0; n], 0, n, 0);

    for i in 0..q {
        println!("{}", answer[i]);
    }
}
