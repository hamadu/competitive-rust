// https://atcoder.jp/contests/abc131/tasks/abc131_f
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
        n: usize,
        points: [(usize1, usize1); n]
    };

    let max = 100000;
    let mut uf = UnionFind::new(max*2);
    for p in points {
        uf.unite(p.0, p.1+max);
    }

    let mut x = vec![0i64; max*2];
    let mut y = vec![0i64; max*2];
    for i in 0..max*2 {
        let id = uf.find(i);
        if i < max {
            x[id] += 1;
        } else {
            y[id] += 1;
        }
    }
    let mut total = 0;
    for i in 0..max*2 {
        total += x[i] * y[i];
    }
    println!("{}", total - n as i64);
}
