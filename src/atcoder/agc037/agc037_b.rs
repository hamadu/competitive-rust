// https://atcoder.jp/contests/agc037/tasks/agc037_b
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

const MOD: u64 = 998244353;

fn main() {
    input! {
        n: u64,
        s: chars
    };

    let mut total = 1u64;
    let mut state = vec![0u64; 8];
    state[0] = n as u64;
    for i in 0..s.len() {
        let v = match s[i] {
            'R' => 1,
            'G' => 2,
            'B' => 4,
             _  => unreachable!("invalid color")
        };
        let w0 = 7 ^ v; // people don't have v
        if state[w0] >= 1 {
            total *= state[w0];
            total %= MOD;
            state[w0] -= 1;
            state[w0^v] += 1;
            continue;
        }

        let mut has1 = false;
        for w1 in vec![6 ^ v, 5 ^ v, 3 ^ v] {
            if w1 != 7 && state[w1] >= 1 {
                has1 = true;
                total *= state[w1];
                total %= MOD;
                state[w1] -= 1;
                state[w1^v] += 1;
                break;
            }
        }
        if has1 {
            continue;
        }

        assert!(state[0] >= 1);
        total *= state[0];
        total %= MOD;
        state[0] -= 1;
        state[v] += 1;
    }
    println!("{}", total);
}

