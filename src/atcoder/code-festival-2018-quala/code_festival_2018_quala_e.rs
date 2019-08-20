// https://atcoder.jp/contests/code-festival-2018-quala/tasks/code_festival_2018_quala_e
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

fn isok(d: i64, n: usize, x: usize, y: usize, favorites: &Vec<(i64, i64)>, values: &Vec<(i64, usize, usize)>) -> bool {
    let per = (x+y)/n;
    let m = values.len();

    let mut min_x = vec![per; n];
    let mut max_x = vec![0; n];
    let mut cnt = vec![0; n];
    let mut okv = 0;
    let mut minv = 0;
    let mut maxv = 0;
    let mut isok = false;


    let mut fr = 0;
    let mut to = 0;
    while fr < m {
        while to < m && values[to].0 - values[fr].0 <= d {
            let (val, pid, xcnt) = values[to];
            let (a, b) = favorites[pid];

            cnt[pid] += 1;
            if cnt[pid] == 1 {
                okv += 1;
            }
            if cnt[pid] == 1 {
                min_x[pid] = xcnt;
                max_x[pid] = xcnt;
                minv += xcnt;
                maxv += xcnt;
            } else if max_x[pid] == xcnt-1 {
                max_x[pid] = xcnt;
                maxv += 1;
            } else {
                assert!(min_x[pid] == xcnt+1);
                min_x[pid] = xcnt;
                minv -= 1;
            }
            to += 1;

            // judge
            if okv == n && minv <= x && x <= maxv {
                return true;
            }
        }

        let pid = values[fr].1;
        let xcnt = values[fr].2;
        cnt[pid] -= 1;
        if cnt[pid] == 0 {
            assert!(max_x[pid] == xcnt && min_x[pid] == xcnt);
            okv -= 1;
            minv -= min_x[pid];
            maxv -= max_x[pid];
            min_x[pid] = per+1;
            max_x[pid] = 0;
        } else if max_x[pid] == xcnt {
            max_x[pid] -= 1;
            maxv -= 1;
        } else {
            assert!(min_x[pid] == xcnt);
            min_x[pid] += 1;
            minv += 1;
        }
        fr += 1;
    }
    false
}

fn main() {
    input! {
        x: usize, y: usize, n: usize, // n|x+y
        favorites: [(i64, i64); n],
    };

    let per = (x+y)/n;
    let mut values = vec![];
    for i in 0..n {
        for j in 0..per+1 {
            let v = favorites[i].0 * (j as i64) + favorites[i].1 * ((per - j) as i64);
            values.push((v, i, j));
        }
    }
    values.sort();

    let mut max = 1e14 as i64;
    let mut min = -1;
    while max - min > 1 {
        let med = (max + min) / 2;
        if isok(med, n, x, y, &favorites, &values) {
            max = med;
        } else {
            min = med;
        }
    }
    println!("{}", max);
}
