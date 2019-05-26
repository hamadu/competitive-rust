// https://atcoder.jp/contests/agc026/tasks/agc026_b
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

fn isok((initial, purchase, threshold, fill): (i64, i64, i64, i64)) -> bool {
    if purchase > fill || purchase > initial {
        return false;
    }
    if threshold >= purchase-1 {
        return true;
    }

    // initial >= purchase > threshold
    assert!(initial >= purchase);
    assert!(purchase > threshold);

    let g = gcd(purchase, fill);
    let tot = initial - threshold;
    let mx = tot / g;

    let mut left = initial - mx * g;
    if left == threshold {
        left += g;
    }
    // threshold < left
    assert!(threshold < left);

    if left < purchase {
        return false;
    }
    true
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

fn main() {
    input! {
        t: usize,
        tests: [(i64, i64, i64, i64); t]
    };

    for t in tests {
        if isok(t) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
