// https://atcoder.jp/contests/agc035/tasks/agc035_d
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

const INF: i64 = 1e18 as i64;

struct Solver {
    n: usize,
    a: Vec<i64>,
    memo: Vec<Vec<HashMap<i64,i64>>>
}

impl Solver {
    fn new(a: Vec<i64>) -> Self {
        let n = a.len();
        Solver {
            n: n,
            a: a,
            memo: dvec!(HashMap::new(); n, n)
        }
    }

    fn solve(&mut self, l: usize, r: usize, al: i64, ar: i64) -> i64 {
        let key = (al<<16)+ar;
        if self.memo[l][r].contains_key(&key) {
            return *self.memo[l][r].get(&key).unwrap();
        }
        let mut ans = INF;
        if l + 1 == r {
            ans = 0;
        } else {
            for i in l+1..r {
                let left = self.solve(l, i, al, al+ar);
                let right = self.solve(i, r, al+ar, ar);
                ans = min(ans, self.a[i] * (al + ar) + left + right);
            }
        }
        self.memo[l][r].insert(key, ans);
        ans
    }
}


fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let ini = a[0] + a[n-1];
    let mut solver = Solver::new(a);
    println!("{}", solver.solve(0, n-1, 1, 1) + ini);
}
