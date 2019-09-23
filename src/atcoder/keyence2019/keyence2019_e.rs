// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_e
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

    ($iter:expr, [ next / $t:tt ]) => {
        {
            let len = read_value!($iter, usize);
            (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        }
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
macro_rules! ifv {
    ($t:expr, $a:expr, $b: expr) => {
        if $t { $a } else { $b }
    }
}

#[allow(unused_macros)]
macro_rules! fill {
    ($t:expr, $v:expr) => {
        for i in 0..$t.len() {
            $t[i] = $v;
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const INF: i64 = 1e18 as i64;

struct UnionFind {
      n: usize,
      parent: Vec<usize>,
      count: Vec<usize>
}

impl UnionFind {
     fn new(n: usize) -> Self {
           let mut p = vec![0; n];
           for i in 0..n {
                 p[i] = i;
           }
           UnionFind { n: n, parent: p, count: vec![1; n] }
     }

     fn find(&mut self, a: usize) -> usize {
           if self.parent[a] == a {
                 return a;
           }
           let par = self.parent[a];
           self.parent[a] = self.find(par);
           self.parent[a]
     }

     fn unite(&mut self, a: usize, b: usize) {
           let a = self.find(a);
           let b = self.find(b);
           if a == b {
                 return;
           }
           let ca = self.count[a];
           let cb = self.count[b];
           let total = ca + cb;
           self.count[b] = total;
           self.count[a] = total;
           if ca < cb {
                 self.parent[a] = b;
           } else {
                 self.parent[b] = a;
           }
     }

     fn same(&mut self, a: usize, b: usize) -> bool {
           self.find(a) == self.find(b)
     }

     fn count(&mut self, a: usize) -> usize {
           let a = self.find(a);
           self.count[a]
     }
}

fn main() {
    input! {
        n: usize, d: i64,
        a: [i64; n]
    };

    let mut throw_cost = vec![];
    let mut receive_cost = vec![];
    for i in 0..n {
        let idx = i as i64;
        throw_cost.push((a[i]+idx*d, i));
        receive_cost.push((a[i]-idx*d, i));
    }

    let cost_func = |u: usize, v: usize| {
        let diff = (max(u, v) - min(u, v)) as i64;
        a[u] + a[v] + d * diff
    };

    throw_cost.sort();
    receive_cost.sort();

    let mut max_idx = 0;
    let mut candidate_edges = vec![];
    let mut best_one = (0, INF);
    for (cost, idx) in throw_cost {
        if idx <= max_idx {
            continue;
        }
        if best_one.1 != INF {
            candidate_edges.push((best_one.0, idx, cost_func(best_one.0, idx)));
        }
        for to in max_idx..idx {
            candidate_edges.push((to, idx, cost_func(to, idx)));
            let recv = a[to]-(to as i64) * d;
            if best_one.1 > recv {
                best_one = (to, recv);
            }
        }
        max_idx = idx;
    }

    let mut max_idx = n-1;
    let mut best_one = (0, INF);
    for (cost, idx) in receive_cost {
        if idx >= max_idx {
            continue;
        }
        if best_one.1 != INF {
            candidate_edges.push((best_one.0, idx, cost_func(best_one.0, idx)));
        }
        for to in idx+1..max_idx+1 {
            candidate_edges.push((to, idx, cost_func(to, idx)));
            let send = a[to]+(to as i64) * d;
            if best_one.1 > send {
                best_one = (to, send);
            }
        }
        max_idx = idx;
    }

    candidate_edges.sort_by_key(|a| a.2);


    let mut uf = UnionFind::new(n);
    let mut total_cost = 0;
    for (u, v, cost) in candidate_edges {
        if uf.same(u, v) {
            continue;
        }
        uf.unite(u, v);
        total_cost += cost;
    }
    assert!(uf.count(0) == n);
    println!("{}", total_cost);
}
