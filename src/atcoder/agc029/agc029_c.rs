// https://atcoder.jp/contests/agc029/tasks/agc029_c
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

fn isok(k: usize, a: &Vec<i32>) -> bool {
    // (char, length)
    let mut d: VecDeque<(usize, i32)> = VecDeque::new();
    let mut len = 0;
    for &ai in a {
        if len < ai {
            d.push_back((0, ai - len));
            len = ai;
        } else {
            while len > ai {
                let mut w = d.pop_back().unwrap();
                if len - w.1 >= ai {
                    len -= w.1;
                    continue;
                }
                w.1 -= len - ai;
                len = ai;
                d.push_back(w);
            }

            // increment one
            let mut kuri = 0;
            while d.len() >= 1 {
                let mut w = d.pop_back().unwrap();
                if w.0 != k - 1 {
                    if w.1 >= 2 {
                        d.push_back((w.0, w.1 - 1));
                    }
                    d.push_back((w.0 + 1, 1));
                    break;
                } else {
                    kuri += w.1;
                }
            }
            if kuri == ai {
                return false;
            }
            if kuri >= 1 {
                d.push_back((0, kuri));
            }
        }
    }
    return true;
}

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    };

    let mut ng = 0;
    let mut ok = n;
    while ok - ng > 1 {
        let med = (ok + ng) / 2;
        if isok(med, &a) {
            ok = med;
        } else {
            ng = med;
        }
    }
    println!("{}", ok);
}
