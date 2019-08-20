// https://atcoder.jp/contests/agc036/tasks/agc036_b
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

#[allow(unused_macros)]
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

const INF: u64 = 1e18 as u64;

fn main() {
    input! {
        n: usize, k: u64,
        a: [usize; n],
    };
    let n64 = n as u64;
    let mut next = dvec!(0; 60, n);
    let mut last = vec![INF; 200001];


    for ii in 0..2*n {
        let i = ii % n;
        if last[a[i]] != INF {
            let li = (last[a[i]] % n64) as usize;
            next[0][li] = ii as u64 - last[a[i]] + 1;
        }
        last[a[i]] = ii as u64;
    }


    for k in 1..60 {
        for i in 0..n {
            let nx = (i as u64 + next[k-1][i]) % n64;
            let nx = nx as usize;
            next[k][i] = min(INF, next[k-1][i]) + min(INF, next[k-1][nx]);

        }
    }

    // println!("{:?}", next[0]);
    // println!("{:?}", next[1]);

    let mut want = n64 * k;
    let mut now = 0;
    let mut k = 59;
    loop {
        let ni = (now % n64) as usize;
        if next[k][ni] <= want {
            want -= next[k][ni];
            now += next[k][ni];
            if want == 0 {
                break;
            }
        } else {
            if k == 0 {
                break;
            }
            k -= 1;
        }
    }

    let mut ans = vec![];
    while want >= 1 {
        let ni = (now % n64) as usize;
        if next[0][ni] <= want {
            want -= next[0][ni];
            now += next[0][ni];
        } else {
            ans.push(a[ni]);
            now += 1;
            want -= 1;
        }
    }

    for i in 0..ans.len() {
        if i >= 1 {
            print!(" {}", ans[i]);
        } else {
            print!("{}", ans[i]);
        }
    }
    println!("");
}
