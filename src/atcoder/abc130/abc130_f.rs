// https://atcoder.jp/contests/abc130/tasks/abc130_f
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

const MAX: f64 = 1e9;
const INF: f64 = 1e18;

type LRDU = (f64, f64, f64, f64);
type Point = (f64, f64, char);
type Range = (f64, f64, f64, f64);

fn minf64(x: f64, y: f64) -> f64 {
    if x <= y {
        x
    } else {
        y
    }
}

fn maxf64(x: f64, y: f64) -> f64 {
    if x >= y {
        x
    } else {
        y
    }
}

fn minmax(points: &Vec<Point>, dir: char) -> LRDU {
    let mut min_x = MAX;
    let mut max_x = -MAX;
    let mut min_y = MAX;
    let mut max_y = -MAX;
    for &pt in points {
        if dir != pt.2 {
            continue;
        }
        min_x = minf64(min_x, pt.0);
        max_x = maxf64(max_x, pt.0);
        min_y = minf64(min_y, pt.1);
        max_y = maxf64(max_y, pt.1);
    }
    (min_x, max_x, min_y, max_y)
}

fn compute_ranges(base: f64, left: f64, right: f64) -> Vec<f64> {
    if left <= minf64(right, base) {
        return vec![0f64, MAX];
    } else if base <= minf64(left, right) {
        let d = (left-base) as f64;
        return vec![0f64, d, MAX];
    } else {
        if right+left <= base*2f64 {
            let center = (right+left)/2f64;
            return vec![0f64, center-right, MAX];
        } else {
            return vec![0f64, base-right, left-base, MAX];
        }
    }
}

fn ptat(pt: &Point, at: f64) -> (f64, f64) {
    let (x, y, dir) = *pt;
    match dir {
        'L' => (x-at, y),
        'R' => (x+at, y),
        'D' => (x, y-at),
        'U' => (x, y+at),
        _ => unreachable!("invalid direction")
    }
}

fn simulate(points: &Vec<Point>, at: f64) -> f64 {
    let mut min_x = MAX;
    let mut max_x = -MAX;
    let mut min_y = MAX;
    let mut max_y = -MAX;
    for &pt in points {
        let (x, y) = ptat(&pt, at);
        min_x = minf64(min_x, x);
        max_x = maxf64(max_x, x);
        min_y = minf64(min_y, y);
        max_y = maxf64(max_y, y);
    }
    (max_x - min_x) * (max_y - min_y)
}

fn main() {
    input! {
        n: usize,
        points: [(f64, f64, chars); n]
    };
    let points = points.into_iter().map(|p| (p.0, p.1, p.2[0])).collect::<Vec<_>>();

    let mut min_x = MAX;
    let mut max_x = -MAX;
    let mut min_y = MAX;
    let mut max_y = -MAX;
    for i in 0..n {
        match points[i].2 {
            'U' | 'D' => {
                min_x = minf64(min_x, points[i].0);
                max_x = maxf64(max_x, points[i].0);
            },
            'L' | 'R' => {
                min_y = minf64(min_y, points[i].1);
                max_y = maxf64(max_y, points[i].1);
            },
            _ => { unreachable!("invalid direction") }
        };
    }

    let lminmax = minmax(&points, 'L');
    let rminmax = minmax(&points, 'R');
    let uminmax = minmax(&points, 'U');
    let dminmax = minmax(&points, 'D');

    let lrange = compute_ranges(min_x, lminmax.0, rminmax.0);
    let rrange = compute_ranges(-max_x, -rminmax.1, -lminmax.1);
    let drange = compute_ranges(min_y, dminmax.2, uminmax.2);
    let urange = compute_ranges(-max_y, -uminmax.3, -dminmax.3);

    let mut best = INF;
    for rv in vec![lrange, rrange, urange, drange] {
        for t in rv {
            best = minf64(best, simulate(&points, t));
        }
    }
    println!("{}", best);
}
