// https://atcoder.jp/contests/agc029/tasks/agc029_b
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

fn solve_odd(a: Vec<u64>) -> usize {
    let mut a = a;
    a.sort();
    a.reverse();

    let mut ans = 0;
    let mut mp: HashMap<u64, usize> = HashMap::new();
    for ai in a {
        let mut has = false;
        for w in 1..32 {
            if ai < 1 << w {
                let rem = (1 << w) - ai;
                let mut h = *mp.get(&rem).unwrap_or(&0);
                if h >= 1 {
                    ans += 1;
                    h -= 1;
                    has = true;
                    mp.insert(rem, h);
                    break;
                }
            }
        }
        if !has {
            let w = mp.get(&ai).unwrap_or(&0) + 1;
            mp.insert(ai, w);
        }
    }
    ans
}

fn solve(a: Vec<u64>) -> usize {
    if a.len() <= 1 {
        return 0;
    }
    let mut odds = vec![];
    let mut rem = vec![];
    for ai in a {
        if ai % 2 == 1 {
            odds.push(ai);
        } else {
            rem.push(ai / 2);
        }
    }
    solve_odd(odds) + solve(rem)
}

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    };

    println!("{}", solve(a));
}
