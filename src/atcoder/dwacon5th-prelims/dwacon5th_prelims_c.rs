// https://atcoder.jp/contests/dwacon5th-prelims/tasks/dwacon5th_prelims_c
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

fn solve(width: usize, s: &Vec<char>, sum_m: &Vec<i64>) -> i64 {
    let n = s.len();
    let mut from = 0;
    let mut to = 0;
    let mut total = 0;

    let mut ds = 0;
    let mut dms = 0;
    while from < n {
        while to < n && to - from < width {
            // add to
            match s[to] {
                'D' => {
                    ds += 1;
                },
                'M' => {
                    dms += ds;
                },
                'C' => {
                    total += dms;
                },
                _ => {},
            };
            to += 1;
        }
        // remove from
        if s[from] == 'D' {
            ds -= 1;
            dms -= sum_m[to] - sum_m[from];
        }
        from += 1;
    }
    total
}

fn main() {
    input! {
        n: usize, s: chars,
        q: usize,
        queries: [usize; q],
    };

    let mut sum_m = vec![0; n+1];
    for i in 0..n {
        let w = sum_m[i];
        if s[i] == 'M' {
            sum_m[i+1] = w+1;
        } else {
            sum_m[i+1] = w;
        }
    }

    for q in queries {
        println!("{}", solve(q, &s, &sum_m));
    }
}
