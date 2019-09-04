// https://atcoder.jp/contests/abc138/tasks/abc138_e
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

fn ctoi(c: char) -> usize {
    if 'a' <= c && c <= 'z' {
        ((c as u8) - ('a' as u8)) as usize
    } else {
        unreachable!("wrong character");
    }
}

fn main() {
    input! {
        s: chars,
        t: chars
    };
    let n = s.len();
    let mut s = s;
    let mut ss = s.clone();
    ss.append(&mut s);

    let mut next = dvec!(0; n, 26);
    let mut last = vec![2*n; 26];
    for i in (0..2*n).rev() {
        if i < n {
            for j in 0..26 {
                next[i][j] = last[j];
            }
        }
        last[ctoi(ss[i])] = i;
    }

    let mut now = 0;
    let mut total = 0;
    let mut ok = true;
    let mut first = true;
    for c in t {
        if first && c == ss[0] {
            first = false;
            continue;
        }
        first = false;

        let to = next[now][ctoi(c)];
        if to == 2*n {
            ok = false;
            break;
        }
        total += to - now;
        now = to % n;
    }
    if ok {
        println!("{}", total+1);
    } else {
        println!("-1");
    }
}
