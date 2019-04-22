// https://atcoder.jp/contests/agc029/tasks/agc029_d
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
        h: usize, w: usize, n: usize,
        objects: [(usize1, usize1); n]
    };

    let mut objects = objects;
    objects.sort();

    let mut ans = h;
    let mut wset = HashSet::new();

    let mut ty = 0;
    let mut lastx = 0;
    for (x, y) in objects {
        let diff = x - lastx;
        if diff > 0 {
            if !wset.contains(&(x - 1, y)) {
                ty += diff - 1;
                if ty >= y {
                    ans = x;
                    break;
                }
                if ty + 1 < y {
                    ty += 1;
                }
            } else {
                ty += 1;
                if ty == y {
                    ty -= 1;
                }
            }
        }
        lastx = x;
        wset.insert((x, y));
    }

    println!("{}", ans);
}
