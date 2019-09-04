// https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_d
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

fn deco(level: &mut Vec<Vec<i32>>, depth: i32, idx: Vec<usize>) {
    if idx.len() == 1 {
        return;
    }
    let n = idx.len();
    let mut a = vec![0; n/2];
    let mut b = vec![0; n-n/2];
    for i in 0..n/2 {
        a[i] = idx[i];
    }
    for i in n/2..n {
        b[i-n/2] = idx[i];
    }

    for &ai in a.iter() {
        for &bi in b.iter() {
            level[ai][bi] = depth;
            level[bi][ai] = depth;
        }
    }
    deco(level, depth+1, a);
    deco(level, depth+1, b);
}

fn print(level: Vec<Vec<i32>>) {
    let n = level.len();
    for i in 0..n-1 {
        for j in i+1..n {
            if j > i+1 {
                print!(" ");
            }
            print!("{}", level[i][j]);
        }
        println!("");
    }
}

fn main() {
    input! {
        n: usize
    };

    let mut level = dvec!(0; n, n);
    let mut idx = vec![0; n];
    for i in 0..n {
        idx[i] = i;
    }
    deco(&mut level, 1, idx);
    print(level);
}
