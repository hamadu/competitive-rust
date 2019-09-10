// https://atcoder.jp/contests/abc136/tasks/abc136_e
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

fn isok(a: &Vec<i32>, d: i32, k: i32) -> bool {
    let n = a.len();
    let mut modulo = vec![];
    for ai in a {
        if ai % d >= 1 {
            modulo.push(ai%d);
        }
    }
    modulo.sort();

    let n = modulo.len();
    let mut sum = vec![0; n+1];
    for i in 0..n {
        sum[i+1] = sum[i] + modulo[i];
    }

    for w in 1..n {
        let mut dw = sum[w];
        let mut up = ((n-w) as i32) * d - (sum[n] - sum[w]);
        if dw == up && dw <= k {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n]
    };
    let sum = a.iter().fold(0, |a,c| a+c);

    let mut divisors = vec![];
    for w in 1..sum+1 {
        if w * w >= sum {
            if w * w == sum {
                divisors.push(w);
            }
            break;
        }
        if sum % w == 0 {
            divisors.push(sum/w);
            divisors.push(w);
        }
    }
    divisors.sort();
    divisors.reverse();

    let mut ans = 1;
    for d in divisors {
        if isok(&a, d, k) {
            ans = d;
            break;
        }
    }
    println!("{}", ans);
}
