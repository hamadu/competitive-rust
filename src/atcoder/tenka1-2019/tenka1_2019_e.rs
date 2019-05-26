// https://atcoder.jp/contests/tenka1-2019/tasks/tenka1_2019_e
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

fn is_zero(a: &Vec<i32>, p: usize) -> bool {
    let n = a.len();
    if a[n - 1] % (p as i32) != 0 {
        return false;
    }
    for i in 1..p {
        let mut sum = 0;
        let mut j = i;
        while j < n {
            sum += a[n - 1 - j];
            j += p - 1;
        }
        if sum.abs() % (p as i32) != 0 {
            return false;
        }
    }
    return true;
}

fn gcd(a: usize, b: usize) -> usize {
    return if a % b == 0 { b } else { gcd(b, a % b) };
}

fn main() {
    input! {
        n: usize,
        a: [i32; n+1]
    };

    let mut g = a[0].abs() as usize;
    for i in 1..n + 1 {
        if a[i] != 0 {
            g = gcd(g, a[i].abs() as usize);
        }
    }

    let mut isp = vec![true; 500000];
    for p in 2..isp.len() {
        if isp[p] {
            let mut pp = p * 2;
            while pp < isp.len() {
                isp[pp] = false;
                pp += p;
            }
            if is_zero(&a, p) {
                println!("{}", p);
            }
            while g % p == 0 {
                g /= p;
            }
        }
    }
    if g >= 2 {
        println!("{}", g);
    }
}
