// https://atcoder.jp/contests/abc139/tasks/abc139_e
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

fn topological_sort(g: Vec<Vec<usize>>) -> i32 {
    let n = g.len();
    let mut indeg = vec![0; n];
    for i in 0..n {
        for &j in &g[i] {
            indeg[j] += 1;
        }
    }
    let mut que = VecDeque::new();
    let mut dp = vec![-1i32; n];
    let mut visited = vec![false; n];
    for i in 0..n {
        if indeg[i] == 0 {
            que.push_back(i);
            dp[i] = 0;
            visited[i] = true;
        }
    }
    while let Some(v) = que.pop_front() {
        let now = dp[v];
        for &j in &g[v] {
            dp[j] = max(dp[j], now+1);
            indeg[j] -= 1;
            if indeg[j] == 0 {
                que.push_back(j);
                visited[j] = true;
            }
        }
    }

    let mut best = 0;
    for i in 0..n {
        if !visited[i] {
            return -1;
        }
        best = max(best, dp[i]);
    }
    best+1
}

fn main() {
    input! {
        n: usize,
        a: [[usize1; n-1]; n]
    };


    let m = n*(n-1)/2;
    let mut g = vec![vec![]; m];

    let mut mid = vec![vec![0; n]; n];
    let mut cnt = 0;
    for i in 0..n {
        for j in i+1..n {
            mid[i][j] = cnt;
            mid[j][i] = cnt;
            cnt += 1;
        }
    }

    for i in 0..n {
        for t in 1..n-1 {
            let j = a[i][t-1];
            let k = a[i][t];
            let u = mid[i][j];
            let v = mid[i][k];
            g[u].push(v);
        }
    }

    println!("{}", topological_sort(g));
}
