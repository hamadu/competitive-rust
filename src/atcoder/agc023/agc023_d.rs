// https://atcoder.jp/contests/agc023/tasks/agc023_d
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
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main() {
    input! {
        n: usize, s: i64,
        a: [(i64, u64); n]
    };

    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    for &m in &a {
        if m.0 < s {
            left.push_front(m);
        } else {
            right.push_back(m);
        }
    }
    // debug!(left, right);

    let mut ans = 0;

    let mut dir = 0;
    while true {
        if left.len() == 0 {
            let rr = right.pop_back().unwrap();
            ans += rr.0 - s;
            break;
        } else if right.len() == 0 {
            let ll = left.pop_back().unwrap();
            ans += s - ll.0;
            break;
        } else {
            let mut ll = left.pop_back().unwrap();
            let mut rr = right.pop_back().unwrap();
            if ll.1 >= rr.1 {
                if dir != 1 {
                    ans += rr.0 - ll.0;
                }
                dir = 1;
                ll.1 += rr.1;
                left.push_back(ll);
            } else {
                if dir != -1 {
                    ans += rr.0 - ll.0;
                }
                dir = -1;
                rr.1 += ll.1;
                right.push_back(rr);
            }
        }
    }
    println!("{}", ans);
}
