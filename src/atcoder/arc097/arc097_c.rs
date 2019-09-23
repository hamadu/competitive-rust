// https://atcoder.jp/contests/arc097/tasks/arc097_c
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

    ($iter:expr, [ next / $t:tt ]) => {
        {
            let len = read_value!($iter, usize);
            (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        }
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
macro_rules! ifv {
    ($t:expr, $a:expr, $b: expr) => {
        if $t { $a } else { $b }
    }
}

#[allow(unused_macros)]
macro_rules! fill {
    ($t:expr, $v:expr) => {
        for i in 0..$t.len() {
            $t[i] = $v;
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const INF: usize = 100000000;

fn main() {
    input! {
        n: usize,
        balls: [(chars, usize1); 2*n]
    };

    let mut position = dvec!(0; 2, n);
    for (&(ref color, index),p) in balls.iter().zip(0..2*n) {
        if color == &vec!['W'] {
            position[0][index] = p;
        } else {
            position[1][index] = p;
        }
    }
    let mut less_than = dvec!(0; 2, n+1, 2*n);
    for c in 0..2 {
        for i in 0..n {
            for j in 0..2*n {
                less_than[c][i+1][j] = less_than[c][i][j];
            }
            let p = position[c][i];
            for j in p+1..2*n {
                less_than[c][i+1][j] += 1;
            }
        }
    }

    let mut dp = dvec!(INF; n+1, n+1);
    dp[0][0] = 0;
    for white in 0..n+1 {
        for black in 0..n+1 {
            let base = dp[white][black];
            if white < n {
                let p = position[0][white];
                dp[white+1][black] = min(dp[white+1][black], base+p-less_than[0][white][p]-less_than[1][black][p]);
            }
            if black < n {
                let p = position[1][black];
                dp[white][black+1] = min(dp[white][black+1], base+p-less_than[0][white][p]-less_than[1][black][p]);
            }
        }
    }

    println!("{}", dp[n][n]);
}
