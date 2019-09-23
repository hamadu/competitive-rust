// https://atcoder.jp/contests/arc101/tasks/arc101_c
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

fn dfs_count(now: usize, par: usize, graph: &Vec<Vec<usize>>, children: &mut Vec<usize>) -> usize {
    let mut count = 1;
    for &to in &graph[now] {
        if to == par {
            continue;
        }
        count += dfs_count(to, now, graph, children);
    }
    children[now] = count;
    count
}

type DpTable = Vec<Vec<Vec<i64>>>;

fn solve(now: usize, par: usize, graph: &Vec<Vec<usize>>, children: &Vec<usize>, dp: &mut DpTable) {
    if children[now] == 1 {
        dp[now][0][0] = 0; // without parent edge
        dp[now][0][1] = 1; // has parent edge
        return;
    }

    let fcn = dp[now][0].len();
    let mut cids = vec![];
    for &to in &graph[now] {
        if to == par {
            continue;
        }
        cids.push((children[to], to));
        solve(to, now, graph, children, dp);
    }


    let mut from = dvec!(0; 2, fcn);
    let mut to = dvec!(0; 2, fcn);
    from[0][1] = 1;

    let mut wl = 1;
    for (cl, cid) in cids {
        for f in 0..2 {
            for w in 1..wl+cl+1 {
                to[f][w] = 0;
            }
        }

        for f in 1..wl+1 {
            for c in 0..cl+1 {
                // 0 <- 00,11
                to[0][f+c] += dp[cid][0][c] * from[0][f] % MOD + dp[cid][1][c] * from[1][f] % MOD;
                to[1][f+c] += dp[cid][1][c] * from[0][f] % MOD + dp[cid][0][c] * from[1][f] % MOD;
            }
        }

        for c in 0..2 {
            for w in 1..wl+cl+1 {
                from[c][w] = to[c][w] % MOD;
            }
        }
        wl += cl;
    }

    dp[now][1][0] = 1;
    for c in 0..2 {
        for f in 1..fcn {
            dp[now][c][f] = from[c][f];
        }
    }

    let mut zero0 = 0;
    let mut zero1 = 0;
    let mut mul = 1;
    for f in 2..fcn {
        if f % 2 == 1 {
            continue;
        }
        mul *= (f-1) as i64;
        mul %= MOD;

        zero0 += mul * dp[now][0][f] % MOD;
        if zero0 >= MOD {
            zero0 -= MOD;
        }
        zero1 += mul * dp[now][1][f] % MOD;
        if zero1 >= MOD {
            zero1 -= MOD;
        }
    }
    dp[now][0][0] = zero1;
    dp[now][1][0] = zero0;
}

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        n: usize,
        edges: [(usize1, usize1); n-1]
    };

    let mut dp = vec![];
    let mut graph = vec![vec![]; n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut children = vec![0; n];

    dfs_count(0, n, &graph, &mut children);

    for i in 0..n {
        let cnt = children[i];
        let wx = dvec!(0; 2, cnt+1);
        dp.push(wx);
    }

    solve(0, n, &graph, &children, &mut dp);

    // debug!(dp);

    println!("{}", (MOD+dp[0][1][0]-dp[0][0][0])%MOD);
}
