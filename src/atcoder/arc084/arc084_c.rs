// https://atcoder.jp/contests/arc084/tasks/arc084_c
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

fn gen(i: usize, n: usize, k: usize, history: Vec<usize>, all: &mut Vec<Vec<usize>>) {
    if history.len() >= 1 {
        all.push(history.clone());
    }
    if i == n {
        return;
    }
    for x in 1..k+1 {
        let mut th = history.clone();
        th.push(x);
        gen(i+1, n, k, th, all);
    }
}

fn rollback(a: &mut Vec<usize>, k: usize, n: usize) {
    let last = a.len()-1;
    if a[last] == 1 {
        a.pop();
    } else {
        a[last] -= 1;
        while a.len() < n {
            a.push(k);
        }
    }
}

fn main() {
    input! {
        k: usize, n: usize
    };

    let mut ans = vec![];
    if k % 2 == 0 {
        ans.push(k/2);
        for i in 1..n {
            ans.push(k);
        }
    } else {
        for i in 0..n {
            ans.push((k+1)/2);
        }
        let back = n/2;
        for i in 0..back {
            rollback(&mut ans, k, n);
        }
    }

    // let mut a = vec![];
    // gen(0, n, k, vec![], &mut a);
    // a.sort();

    // let len = a.len();
    // debug!(len, a[(len+1)/2-1]);
    // assert_eq!(ans, a[(len+1)/2-1]);

    let line = ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", line);
}
