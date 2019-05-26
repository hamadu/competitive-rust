// https://atcoder.jp/contests/agc027/tasks/agc027_b
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

// fn range_cost(l: usize, r: usize, sum: &Vec<Vec<u64>>) -> u64 {
//     let s = sum[1][r] - sum[1][l];
//     let t = sum[0][r] - sum[0][l];
//     s - t * l as u64 * 2
// }

fn main() {
    input! {
        n: usize, t: u64,
        x: [u64; n]
    };

    let mut sum = vec![0; n + 1];
    for i in 1..n + 1 {
        let s0 = sum[i - 1];
        sum[i] = s0 + x[i - 1];
    }

    let mut ans = 1e18 as u64;
    let base_cost = sum[n] * 5 + t * (n as u64);

    for cur in 1..n + 1 {
        if cur * 2 > n {
            let mut cost = cur as u64 * t;
            ans = min(ans, cost);
            break;
        }
        let left = n - cur * 2;

        let dan = left / cur;
        let par = left % cur;

        assert!(par + dan * cur == left);

        let mut cost = 0;
        if par >= 1 {
            cost += (2 * (dan + 1)) as u64 * sum[par];
        }
        for d in 0..dan {
            let l = par + cur * d;
            let r = par + cur * (d + 1);
            cost += (2 * (dan - d)) as u64 * (sum[r] - sum[l]);
        }
        cost += cur as u64 * t;
        ans = min(ans, cost);
    }
    ans += base_cost;
    println!("{}", ans);
}
