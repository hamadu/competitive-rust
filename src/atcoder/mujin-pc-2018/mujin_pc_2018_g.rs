// https://atcoder.jp/contests/mujin-pc-2018/tasks/mujin_pc_2018_g
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
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
type Query = (i64, i64, i64, i64, i64, i64, i64);

const LIMIT: i64 = 1e18 as i64;

const MOD: i64 = 998244353;

fn solve_all(k: i64) -> i64 {
    if k < 0 {
        0
    } else {
        k
    }
}

fn solve((x1, y1, x2, y2, x3, y3, k): Query) -> i64 {
    (solve_all(k) - solve_main((x1, y1, x2, y2, x3, y3, k)) + MOD) % MOD;
}

fn solve_main((x1, y1, x2, y2, x3, y3, k): Query) -> i64 {
    let mut b0 = y1*x3-x1*y3;
    let mut c0 = -(y1*x2-x1*y2);
    let gbc = gcd(b0.abs(), c0.abs());
    assert!(gbc != 0);
    b0 /= gbc;
    c0 /= gbc;

    let mut a1 = y2*x3-x2*y3;
    let mut c1 = -(x1*y2-x2*y1);
    let gbc = gcd(a1.abs(), c1.abs());
    assert!(gbc != 0);
    a1 /= gbc;
    c1 /= gbc;

    let gcc = gcd(c0.abs(), c1.abs());
    c0 /= gcc;
    c1 /= gcc;
    if (c0 > LIMIT / c1 || a1 > LIMIT / c0 || b0 > LIMIT / c1) {
        return 0;
    }
    let mut a = a1 * c0;
    let mut b = b0 * c1;
    let mut c = c0 * c1;
    let gabc = gcd(gcd(a.abs(), b.abs()), c.abs());
    a /= gabc;
    b /= gabc;
    c /= gabc;

    if a+b+c < 0  {
        a *= -1;
        b *= -1;
        c *= -1;
    }

    // 0 <= n1+a <= k
    // 0 <= n2+b <= k
    // 0 <= n3+c <= k
    // 0 <= n1+n2+n3+a+b+c <= k
    // a+b+c >= 0

    let large = max(a, max(b, c));
    let small = min(a, min(b, c));
    let med = a + b + c - large - small;
    if large > k {
        return 0;
    } else if small < -k {
        return 0;
    }

    if small >= 0 {
        return solve_all(k-small-med-large);
    } else if med >= 0 {
        if med+large > k
        // w = small to k
        // n1+n2 <= (k-med-large, k-n3)
    } else {
        let lk = k + small + med;
        if lk < 0 {
            0
        } else {

        }
    }



}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

fn main() {
    input! {
        q: usize,
        queries: [(i64, i64, i64, i64, i64, i64, i64); q]
    };

    for q in queries {
        println!("{}", solve(q));
    }
}
