// https://atcoder.jp/contests/abc135/tasks/abc135_e
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

fn output(ans: Vec<(i64, i64)>, xf: i64, yf: i64) {
    if ans.len() == 0 {
        println!("-1");
    } else {
        println!("{}", ans.len());
        for &(x, y) in ans.iter().rev() {
            let x = x * xf;
            let y = y * yf;
            println!("{} {}", x, y);
        }
    }
}

fn sw(s: i64, x: (i64, i64)) -> (i64, i64) {
    if s == 1 {
        (x.1, x.0)
    } else {
        x
    }
}

fn main() {
    input! {
        k: i64,
        x: i64, y: i64
    };

    let xa = if x < 0 { -1 } else { 1 };
    let ya = if y < 0 { -1 } else { 1 };
    let mut x = x.abs();
    let mut y = y.abs();
    if x + y == k {
        output(vec![(x, y)], xa, ya);
        return;
    }
    let mut v1 = vec![];
    let mut wk = 2*k;

    if k % 2 == 0 {
        if (x.abs()+y.abs()) % 2 == 1 {
            output(vec![], xa, ya);
            return;
        }
    }
    while x + y > wk {
        v1.push((x, y));
        if x >= k {
            x -= k;
            continue;
        }
        if y >= k {
            y -= k;
            continue;
        }
    }
    v1.push((x, y));

    let swp = if y > x { 1 } else { 0 };
    let mut y = y;
    let mut x = x;
    if y > x {
        let t = y;
        y = x;
        x = t;
    }

    if k % 2 == 0 || (x + y) % 2 == 0 {
        if x + y > k {
            let total = (x + (k - y) - k) / 2;
            v1.push(sw(swp, (total, k - total)));
        } else {
            let total = (x - y) / 2;
            v1.push(sw(swp, (total, k - total)));
        }
    } else {
        let wx = x;
        let wy = y - k;

        let total = (- wx - wy) / 2;
        if total >= 0 {
            v1.push(sw(swp, (wx + total, wy + total)));
        } else {
            let wx = x - k;
            let wy = y;
            let total = (- wx + wy) / 2;
            v1.push(sw(swp, (wx + total, wy - total)));
        }
        v1.push(sw(swp, (k, 0)));
    }
    output(v1, xa, ya);
}
