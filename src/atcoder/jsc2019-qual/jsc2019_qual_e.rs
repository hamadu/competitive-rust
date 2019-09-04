// https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_e
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

struct UnionFind {
    n: usize,
    parent: Vec<usize>,
    count: Vec<usize>,
    flag: Vec<bool>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = i;
        }
        UnionFind { n: n, parent: p, count: vec![1; n], flag: vec![false; n] }
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
        let tf = self.flag[a] | self.flag[b];
        self.flag[a] = tf;
        self.flag[b] = tf;
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
        n: usize, h: usize, w: usize,
        cards: [(usize1, usize1, i64); n]
    };
    let mut cards = cards;
    for i in 0..n {
        cards[i].1 += h;
    }
    cards.sort_by_key(|c| -c.2);


    let mut score = 0;
    let mut uf = UnionFind::new(h+w);
    for c in cards {
        if uf.same(c.0, c.1) {
            let cid = uf.find(c.0);
            if uf.flag[cid] {
                continue;
            }
            uf.flag[cid] = true;
            score += c.2;
        } else {
            let cid1 = uf.find(c.0);
            let cid2 = uf.find(c.1);
            if uf.flag[cid1] && uf.flag[cid2] {
                continue;
            }
            uf.unite(c.0, c.1);
            score += c.2;
        }
    }
    println!("{}", score);
}
