// https://atcoder.jp/contests/agc028/tasks/agc028_d
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

const MOD: u64 = 1000000007;

const INF: usize = 1000000;

fn within(k: usize, l: usize, r: usize) -> bool {
    return l <= k && k <= r;
}

fn fillways(l: usize, r: usize, emptysum: &Vec<usize>, fw: &Vec<u64>) -> u64 {
    let e = emptysum[r + 1] - emptysum[l];
    fw[e]
}

fn solve(
    l: usize,
    r: usize,
    dp: &mut Vec<Vec<u64>>,
    counterpart: &Vec<usize>,
    emptysum: &Vec<usize>,
    fw: &Vec<u64>,
) -> u64 {
    if dp[l][r] != MOD {
        return dp[l][r];
    }
    let n = counterpart.len();
    let free = emptysum[r + 1] - emptysum[l];
    let mut ret = 1;
    for i in 0..n {
        if counterpart[i] == INF {
            continue;
        }
        if !within(i, l, r) && within(counterpart[i], l, r) {
            ret = 0;
        }
        if within(i, l, r) && !within(counterpart[i], l, r) {
            ret = 0;
        }
    }
    if free % 2 == 1 {
        ret = 0;
    }
    if ret == 0 {
        dp[l][r] = ret;
        return 0;
    }

    ret = fillways(l, r, emptysum, fw);

    let mut sub = 0;
    for ci in l + 1..r {
        let s =
            solve(l, ci, dp, counterpart, emptysum, fw) * fillways(ci + 1, r, emptysum, fw) % MOD;
        sub += s;
        sub %= MOD;
    }

    ret += MOD - sub;
    ret %= MOD;

    dp[l][r] = ret;
    return dp[l][r];
}

fn main() {
    input! {
        n: usize, k: usize,
        edges: [(usize1, usize1); k]
    };

    let mut fw = vec![0; 2 * n + 1];
    {
        fw[0] = 1;
        let mut w = 2;
        while w <= 2 * n {
            let wo = fw[w - 2] * ((w - 1) as u64) % MOD;
            fw[w] = wo;
            w += 2;
        }
    }

    let mut counterpart = vec![INF; 2 * n];
    for e in edges.iter() {
        counterpart[e.0] = e.1;
        counterpart[e.1] = e.0;
    }
    let mut emptysum = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        let s = emptysum[i] + if counterpart[i] == INF { 1 } else { 0 };
        emptysum[i + 1] = s;
    }

    let mut dp = vec![vec![MOD; 2 * n]; 2 * n];
    let mut ans = 0;
    for i in 0..2 * n {
        for j in i + 1..2 * n {
            let mut fr = 0;
            for k in 0..2 * n {
                if !within(k, i, j) && counterpart[k] == INF {
                    fr += 1;
                }
            }

            let fr = fw[fr];
            ans += solve(i, j, &mut dp, &counterpart, &emptysum, &fw) * fr % MOD;
            //println!(
            //    "{} {} {} {}",
            //    i,
            //    j,
            //    solve(i, j, &mut dp, &counterpart, &emptysum, &fw),
            //    fr
            //);

            ans %= MOD;
        }
    }

    println!("{}", ans);
}
