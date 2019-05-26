// https://atcoder.jp/contests/agc026/tasks/agc026_c
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

fn compute(a: Vec<char>) -> HashMap<(Vec<char>, Vec<char>), u64> {
    let mut map = HashMap::new();
    let n = a.len();
    for p in 0..(1<<n) {
        let mut red = vec![];
        let mut blue = vec![];
        for i in 0..n {
            if p & (1<<i) == 0 {
                red.push(a[i]);
            } else {
                blue.push(a[i]);
            }

        }
        let pair = (red, blue);
        let cnt = *(map.get(&pair).unwrap_or(&0)) + 1;
        map.insert(pair, cnt);
    }
    map
}

fn main() {
    input! {
        n: usize,
        s: chars
    };

    let mut left = vec!['-'; n];
    let mut right = vec!['-'; n];

    left.copy_from_slice(&s[0..n]);
    right.copy_from_slice(&s[n..2*n]);
    right.reverse();

    let leftMap = compute(left);
    let rightMap = compute(right);

    let mut ans = 0;
    for ((l, r), lways) in leftMap {
        let rways = *rightMap.get(&(r.to_vec(), l.to_vec())).unwrap_or(&0);
        ans += lways * rways;
    }

    // ABcd|DCba
    println!("{}", ans);
}
