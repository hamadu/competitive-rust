// https://atcoder.jp/contests/code-festival-2018-final-open/tasks/code_festival_2018_final_d
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
    if 'A' <= c && c <= 'Z' {
        c as usize - 'A' as usize
    } else if 'a' <= c && c <= 'z' {
        c as usize - 'a' as usize + 26
    } else {
        unreachable!("wrong character");
    }
}

fn itoc(c: usize) -> char {
    if 0 <= c && c < 26 {
        (c as u8 + 'A' as u8) as char
    } else if c < 52 {
        (c as u8 - 26 + 'a' as u8) as char
    } else {
        unreachable!("wrong character");
    }
}


fn main() {
    input! {
        n: usize,
        s: [chars; n]
    };
    let t = s.into_iter().map(|si| si.into_iter().map(|c| ctoi(c)).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut ans = dvec!(0; 52, 52, 52);

    let mut upd = dvec!(n; 52, 52, 52);
    for si in 0..n {
        let ti = &t[si];
        let n = ti.len();
        let mut left = vec![0; n+1];
        let mut right = vec![0; n+1];
        for i in 0..n {
            left[i+1] = left[i] | (1i64<<(ti[i] as i64));
            right[n-i-1] = right[n-i] | (1i64<<(ti[n-i-1] as i64));
        }
        for i in 0..n {
            let lt = left[i];
            let rt = right[i+1];
            for l in 0..52 {
                for r in 0..52 {
                    if lt & (1i64<<l) >= 1 && rt & (1i64<<r) >= 1 {
                        if upd[l][ti[i]][r] != si {
                            upd[l][ti[i]][r] = si;
                            ans[l][ti[i]][r] += 1;
                        }
                    }
                }
            }
        }
    }


    let mut best = (0, 0, 0, 0);
    for i in 0..52 {
        for j in 0..52 {
            for k in 0..52 {
                if best.0 < ans[i][j][k] {
                    best = (ans[i][j][k], i, j, k);
                }
            }
        }
    }
    println!("{}{}{}", itoc(best.1), itoc(best.2), itoc(best.3));
}
