// https://atcoder.jp/contests/agc032/tasks/agc032_a
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

fn main() {
    input! {
        n: usize,
        b: [usize; n]
    };
    let mut isok = true;
    let mut b = b;

    let mut ans = vec![0; n];
    while b.len() >= 1 {
        let mut n = b.len();
        let mut found = false;
        while n >= 1 {
            n -= 1;
            if b[n] == n + 1 {
                ans[b.len() - 1] = n + 1;
                let mut nb = vec![];
                for i in 0..b.len() {
                    if i != n {
                        nb.push(b[i]);
                    }
                }
                b = nb;
                found = true;
                break;
            }
        }
        if !found {
            isok = false;
            break;
        }
    }
    if isok {
        for x in ans {
            println!("{}", x);
        }
    } else {
        println!("-1")
    }
}
