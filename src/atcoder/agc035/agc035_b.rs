// https://atcoder.jp/contests/agc035/tasks/agc035_b
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

#[allow(unused_macros)]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
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


fn dfs(now: usize, par: usize, parent: &mut Vec<usize>, tree: &Vec<Vec<usize>>, outdeg: &Vec<usize>, flag: &mut Vec<usize>) {
    parent[now] = par;
    for &to in &tree[now] {
        if to == par {
            continue;
        }
        dfs(to, now, parent, tree, outdeg, flag);
    }
    if outdeg[now] % 2 == 1 {
        flag[now] += 1;
    }
    if par != tree.len() {
        flag[par] += flag[now];
    }
}

fn verify(n: usize, edges: &Vec<(usize, usize)>) -> bool {
    let mut ok = true;
    let mut deg = vec![0; n];
    for &e in edges {
        deg[e.0] += 1;
    }
    for i in 0..n {
        if deg[i] % 2 == 1 {
            return false
        }
    }
    true
}

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(usize1, usize1); m],
    };

    let mut uf = UnionFind::new(n);
    let mut tree = vec![vec![]; n];
    let mut outtree = vec![];
    for &e in &edges {
        if uf.same(e.0, e.1) {
            outtree.push(e);
            continue;
        }
        uf.unite(e.0, e.1);
        tree[e.0].push(e.1);
        tree[e.1].push(e.0);
    }

    let mut outdeg = vec![0; n];
    for i in 0..n {
        outdeg[i] = tree[i].len();
        if i != 0 {
            outdeg[i] -= 1;
        }
    }

    let mut ans = vec![];
    for &e in &outtree {
        ans.push(e);
        outdeg[e.0] += 1;
    }

    let mut parent = vec![0; n];
    let mut flag = vec![0; n];

    // println!("{:?}", outdeg);

    dfs(0, n, &mut parent, &tree, &mut outdeg, &mut flag);

    // println!("{:?} {:?}", parent, flag);

    for i in 1..n {
        if flag[i] % 2 == 1 {
            ans.push((i, parent[i]));
        } else {
            ans.push((parent[i], i));
        }
    }

    if verify(n, &ans) {
        for &e in &ans {
            println!("{} {}", e.0+1, e.1+1);
        }
    } else {
        println!("-1");
    }
}
