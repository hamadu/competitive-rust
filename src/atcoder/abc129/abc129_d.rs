// https://atcoder.jp/contests/abc129/tasks/abc129_d
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

fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h]
    };

    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    let mut wo = dvec!(0; 4, h, w);
    for d in 0..4 {
        if d < 2 {
            for i in 0..h {
                for j in 0..w {
                    let tj = ifv!(d == 0, j, w-1-j);
                    wo[d][i][tj] = ifv!(d == 0,
                        ifv!(tj >= 1, wo[d][i][tj-1], 0),
                        ifv!(tj < w-1, wo[d][i][tj+1], 0)
                    );
                    if s[i][tj] == '#' {
                        wo[d][i][tj] = 0;
                    } else {
                        wo[d][i][tj] += 1;
                    }
                }
            }
        } else {
            for j in 0..w {
                for i in 0..h {
                    let ti = ifv!(d == 2, i, h-1-i);
                    wo[d][ti][j] = ifv!(d == 2,
                        ifv!(ti >= 1, wo[d][ti-1][j], 0),
                        ifv!(ti < h-1, wo[d][ti+1][j], 0)
                    );
                    if s[ti][j] == '#' {
                        wo[d][ti][j] = 0;
                    } else {
                        wo[d][ti][j] += 1;
                    }
                }
            }
        }
    }

    // debug!(wo);

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let mut dir = 0;
                for d in 0..4 {
                    dir += wo[d][i][j]-1;
                }
                ans = max(ans, dir+1);
            }
        }
    }
    println!("{}", ans);
}
