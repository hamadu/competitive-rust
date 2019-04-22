// https://atcoder.jp/contests/agc030/tasks/agc030_b
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

fn solve(x: Vec<u64>, all: u64) -> u64 {
    let n = x.len();
    if n == 1 {
        return x[0];
    }
    let mut ldsum = 0;
    let mut rdsum = 0;
    let mut left = vec![];
    let mut right = vec![];
    {
        let mut l = 0;
        let mut r = n - 1;
        while l <= r {
            left.push(x[l]);
            ldsum += x[l];
            if l != r {
                right.push(x[r]);
                rdsum += all - x[r];
            }
            l += 1;
            r -= 1;
        }
    }
    let mut ans = 0;

    let mut rt = right.len();
    for lh in 0..n {
        if lh >= 1 {
            ldsum -= left[lh - 1];
        }
        let mut lcnt = left.len() - lh;
        let mut rcnt = rt;
        if lcnt < rcnt {
            left.push(right[rt - 1]);
            rdsum -= all - right[rt - 1];
            ldsum += right[rt - 1];
            rt -= 1;
            lcnt += 1;
            rcnt -= 1;
        }
        let mut distances = 0;
        if lcnt == rcnt {
            // last = R
            distances += (ldsum + rdsum) * 2;
            distances -= all - right[rt - 1];
        } else if lcnt == rcnt + 1 {
            // last = L
            distances += (ldsum + rdsum) * 2;
            let ll = left.len();
            distances -= left[ll - 1];
        } else {
            panic!("arien");
        }
        ans = max(ans, distances);
    }
    ans
}

fn main() {
    input! {
        l: u64, n: usize,
        x: [u64; n]
    };

    let mut revx = vec![0; n];
    for i in 0..n {
        revx[i] = l - x[n - 1 - i];
    }

    let ans = max(solve(x, l), solve(revx, l));
    println!("{}", ans);
}
