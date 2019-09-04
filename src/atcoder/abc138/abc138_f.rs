// https://atcoder.jp/contests/abc138/tasks/abc138_f
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

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        l: i64, r: i64
    };



    let mut dp = dvec!(0; 60, 2, 2, 2);
    dp[59][0][0][0] = 1;

    let mut answer = 0;
    for idx in (0..60).rev() {
        let f = 1i64<<idx;
        let hasr = (r & f) == f;
        let hasl = (l & f) == f;
        for on in 0..2 {
            for lower_r in 0..2 {
                for upper_l in 0..2 {
                    let base = dp[idx][on][lower_r][upper_l];
                    if base == 0 {
                        continue;
                    }
                    for x in 0..2 {
                        for y in x..2 {
                            if upper_l == 0 && hasl && x == 0 {
                                // x is under L.
                                continue;
                            }
                            if lower_r == 0 && !hasr && y == 1 {
                                // y is over R.
                                continue;
                            }
                            if on == 0 && x != y {
                                // leftmost significant bit is different.
                                continue;
                            }

                            let mut next_on = on;
                            let mut next_upper = upper_l;
                            let mut next_lower = lower_r;
                            if !hasl && x == 1 {
                                next_upper = 1;
                            }
                            if hasr && y == 0 {
                                next_lower = 1;
                            }
                            if x == 1 && y == 1 {
                                next_on = 1;
                            }

                            if idx >= 1 {
                                dp[idx-1][next_on][next_lower][next_upper] += base;
                                dp[idx-1][next_on][next_lower][next_upper] %= MOD;
                            } else {
                                answer += base;
                                answer %= MOD;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", answer);
}
