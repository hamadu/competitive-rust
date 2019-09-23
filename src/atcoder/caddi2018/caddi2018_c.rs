// https://atcoder.jp/contests/caddi2018/tasks/caddi2018_c
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

fn step(mut a: i64, mut b: i64) -> (i64, bool) {
    if a <= b {
        (0, a == b)
    } else {
        let mut cnt = 0;
        while b < a {
            b *= 4;
            cnt += 1;
        }
        (cnt, a == b)
    }
}

fn diff(mut a: i64, mut b: i64) -> i64 {
    let mut d = 0;
    if a > b {
        while b < a {
            b *= 4;
            d += 1;
        }
        -d
    } else {
        while a <= b {
            a *= 4;
            d += 1;
        }
        d - 1
    }
}

fn form_p4(mut a: i64) -> (i64, i64) {
    let mut a4 = 1;
    while a >= 4 {
        a /= 4;
        a4 += 1;
    }
    (a4, a)
}

/// Returns minimum pb such that a * 4^pa <= b * 4^pb holds.
fn compare(mut a: i64, mut pa: i64, mut b: i64) -> i64 {
    let dab = diff(a, b) - pa;
    if dab < 0 {
        -dab
    } else {
        0
    }
}

const INF: i64 = 1e12 as i64;
const MAX_POW: usize = 16;

fn solve(a: Vec<i64>) -> Vec<i64> {
    let n = a.len();

    let mut dp = dvec!(0i64; n+1, MAX_POW);
    for i in 0..MAX_POW {
        dp[n-1][i] = i as i64;
    }

    for i in (0..n-1).rev() {
        for d in 0..MAX_POW {
            let req = compare(a[i], d as i64, a[i+1]) as usize;
            if req < MAX_POW {
                dp[i][d] = (d as i64) + dp[i+1][req];
            } else {
                dp[i][d] = (d as i64) + (dp[i+1][15] + ((req - 15) * (n - i - 1)) as i64);
            }
        }
    }

    let mut ret = vec![0; n+1];
    for i in 0..n+1 {
        ret[i] = dp[i][0];
    }
    ret
}


fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let front = solve(a.clone());
    let mut a = a;
    for i in 0..n {
        a[i] *= 2;
    }
    a.reverse();
    let back = solve(a);

    // debug!(front,back);

    let mut best = INF;
    for f in 0..n {
        let take_f = n-f;
        let take_b = n-take_f;
        let b = n-take_b;

        best = min(best, 2 * (front[f] + back[b]) + (take_b as i64));
    }
    println!("{}", best);
}
