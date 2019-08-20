// https://atcoder.jp/contests/agc034/tasks/agc034_e
//
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::fmt::*;
use std::io::*;
use std::str::*;

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

const INF: u64 = 1e12 as u64;

type Graph = Vec<Vec<usize>>;

struct Solver {
    tree: Graph,
    s: Vec<u64>,
    weight: Vec<u64>,
    count: Vec<u64>,
}

impl Solver {
    fn dfs(&mut self, now: usize, par: usize) -> (u64, u64) {
        let mut weight = 0;
        let mut count = self.s[now];
        for to in self.tree[now].clone() {
            if to == par {
                continue;
            }
            let (w, c) = self.dfs(to, now);
            weight += w + c;
            count += c;
        }
        self.weight[now] = weight;
        self.count[now] = count;
        (weight, count)
    }

    fn find_most(&mut self, root: usize) -> (usize, u64) {
        let mut most = 0;
        let mut rt = 0;
        let all = self.weight[root];
        for to in self.tree[root].clone() {
            if all < self.weight[to] {
                continue;
            }
            let cw = self.weight[to] + self.count[to];
            if most < cw {
                most = cw;
                rt = to;
            }
        }
        (rt, most)
    }

    fn dfs2(&mut self, now: usize, par: usize) -> u64 {
        let mut most = 0;
        let mut count = 0;
        for to in self.tree[now].clone() {
            if par == to {
                continue;
            }
            count += self.dfs2(to, now);

            let cw = self.weight[to] + self.count[to];
            if most < cw {
                most = cw;
            }
        }
        self.weight[now] -= count * 2;
        if most * 2 <= self.weight[now] {
            count += self.weight[now] / 2;
            self.weight[now] %= 1;
        } else {
            let left = self.weight[now] - most;
            count += left;
            self.weight[now] -= left * 2;
        }
        count
    }

    fn solve(&mut self, root: usize) -> u64 {
        let n = self.tree.len();
        self.dfs(root, n);

        let all = self.weight[root];
        if all % 2 == 1 {
            return INF;
        }

        let (rt, most) = self.find_most(root);
        if most * 2 <= all {
            return all / 2;
        }
        let raise = (2 * most - all) / 2;
        let max_count = self.dfs2(rt, root);
        if max_count >= raise {
            return all / 2;
        }
        return INF;
    }
}


fn main() {
    input! {
        n: usize,
        s: chars,
        edges: [(usize1, usize1); n-1],
    };
    let mut tree = vec![vec![]; n];
    for &e in &edges {
        tree[e.0].push(e.1);
        tree[e.1].push(e.0);
    }
    let si = s.iter().map(|&c| (c as u64) - ('0' as u64)).collect();
    let mut cost = INF;
    let mut slv = Solver { tree: tree, s: si, weight: vec![0; n], count: vec![0; n] };
    for i in 0..n {
        cost = min(cost, slv.solve(i));
    }
    if cost == INF {
        println!("-1");
    } else {
        println!("{}", cost);
    }
}
