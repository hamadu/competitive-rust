// https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_c
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
        n: usize,
        s: chars
    };
    assert!(s.len() == 2*n);

    let mut t = vec![0i64; 2*n];
    let mut f = 0;
    let mut isok = true;
    for i in 0..2*n {
        if s[i] == 'B' {
            if f % 2 == 0 {
                f += 1;
            } else {
                f -= 1;
                t[i] = 1;
            }
        } else {
            if f % 2 == 0 {
                f -= 1;
                t[i] = 1;
            } else {
                f += 1;
            }
        }
        if f < 0 {
            isok = false;
        }
    }
    if f != 0 {
        isok = false;
    }
    if isok {
        let mut ans = 1i64;
        let mut w = 0i64;
        for i in (0..2*n).rev() {
            if t[i] == 1 {
                w += 1;
            } else {
                ans *= w;
                ans %= MOD;
                w -= 1;
            }
        }
        for i in 1..n+1 {
            ans *= i as i64;
            ans %= MOD;
        }
        println!("{}", ans);
    } else {
        println!("0");
    }
}
