// https://atcoder.jp/contests/dwacon5th-prelims/tasks/dwacon5th_prelims_d
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

fn isok(size: usize, freq: &Vec<Vec<usize>>, sum: &mut Vec<Vec<i64>>, d: usize) -> bool {
    let (base, md) = (size / d, size % d);
    let max = (base+1)*(base+1);
    let med = (base+1)*base;
    let min = base*base;
    for i in 0..d {
        for j in 0..d {
            if freq[i][j] > max {
                return false;
            }
        }
    }
    for i in 0..2*d {
        for j in 0..2*d {
            let v = if freq[i%d][j%d] <= min {
                1 // happy
            } else if freq[i%d][j%d] <= med {
                0 // soso
            } else {
                -1e9  as i64 // not good
            };
            sum[i+1][j+1] = sum[i+1][j] + sum[i][j+1] + v - sum[i][j];
        }
    }

    // x0 <= x1, y0 <= y1
    let range_sum = |x0: usize, y0: usize, x1: usize, y1: usize| {
        assert!(x0 <= x1);
        assert!(y0 <= y1);
        sum[x1][y1] - sum[x1][y0] - sum[x0][y1] + sum[x0][y0]
    };

    for i in 0..d {
        for j in 0..d {
            let topright = range_sum(i+md, j+md, i+d, j+d);
            if topright != ((d-md) * (d-md)) as i64 {
                continue;
            }
            if range_sum(i, j+md, i+md, j+d) < 0 {
                continue;
            }
            if range_sum(i+md, j, i+d, j+md) < 0 {
                continue;
            }
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize, d: usize,
        plushies: [(usize, usize); n]
    };

    let mut tbl = vec![vec![0; 2*d+1]; 2*d+1];
    let mut freq = vec![vec![0; d]; d];
    let mut maxstack = 0;
    for (x, y) in plushies {
        freq[x%d][y%d] += 1;
        maxstack = max(maxstack, freq[x%d][y%d]);
    }

    let mut ng = 0;
    let mut ok = maxstack * (d + 2);
    while ok - ng > 1 {
        let med = (ok + ng) / 2;
        if isok(med, &freq, &mut tbl, d) {
            ok = med;
        } else {
            ng = med;
        }
    }
    println!("{}", ok-1);
}
