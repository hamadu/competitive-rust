// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_c
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

fn main() {
    input! {
        n: usize, s: [chars; n]
    };

    let mut begin_b = 0;
    let mut end_a = 0;
    let mut both = 0;
    let mut total = 0;

    for i in 0..n {
        let m = s[i].len();
        for j in 0..m-1 {
            if s[i][j] == 'A' && s[i][j+1] == 'B' {
                total += 1;
            }
        }
        if s[i][0] == 'B' {
            begin_b += 1;
        }
        if s[i][m-1] == 'A' {
            end_a += 1;
        }
        if s[i][0] == 'B' && s[i][m-1] == 'A' {
            both += 1;
        }
    }

    total += min(begin_b, end_a);
    if min(begin_b, end_a) >= 1 && begin_b == end_a && begin_b == both {
        total -= 1;
    }
    println!("{}", total);
}
