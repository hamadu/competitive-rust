// https://atcoder.jp/contests/abc129/tasks/abc129_f
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

#[derive(Debug, Clone)]
struct SquareMatrix {
    n: usize,
    data: Vec<Vec<i64>>,
    modulo: i64
}

impl SquareMatrix {
    fn new(data: Vec<Vec<i64>>, modulo: i64) -> SquareMatrix {
        let n = data.len();
        SquareMatrix { n: n, data: data, modulo: modulo }
    }

    fn eye(n: usize, modulo: i64) -> SquareMatrix {
        let mut data = dvec!(0; n, n);
        for i in 0..n {
            data[i][i] = 1;
        }
        SquareMatrix { n: n, data: data, modulo: modulo }
    }

    fn mul(u: i64, v: i64, modulo: i64) -> i64 {
        u * v % modulo
    }

    fn add(u: i64, v: i64, modulo: i64) -> i64 {
        (u + v) % modulo
    }

    fn multiply(left: &SquareMatrix, other: &SquareMatrix) -> SquareMatrix {
        assert!(left.n == other.n);
        assert!(left.modulo == other.modulo);
        let modulo = left.modulo;
        let n = left.n;
        let mut data = dvec!(0; n, n);
        for i in 0..n {
            for j in 0..n {
                let mut w = 0;
                for k in 0..n {
                    w = Self::add(w, Self::mul(left.data[i][k], other.data[k][j], modulo), modulo);
                }
                data[i][j] = w;
            }
        }
        SquareMatrix { n: n, data: data, modulo: modulo }
    }

    fn pow(self, mut p: i64) -> SquareMatrix {
        let n = self.n;
        let mut ret = Self::eye(n, self.modulo);
        let mut a = self;
        while p >= 1 {
            if p & 1 == 1 {
                ret = Self::multiply(&ret, &a);
            }
            a = Self::multiply(&a, &a);
            p >>= 1;
        }
        ret
    }
}

fn indices(from: i64, to: i64, a: i64, b: i64) -> (i64, i64) {
    let from = max(from, a);
    if to < a {
        return (-1, -1);
    }
    let mut begin_idx = (from - a) / b + ifv!((from - a) % b == 0, 0, 1);
    if from <= a {
        begin_idx = 0;
    }
    let end_idx = (to - a) / b;
    (begin_idx, end_idx)
}

struct Combination {
    fact: Vec<u64>,
    invfact: Vec<u64>,
}

fn pow(a: i64, p: i64, modulo: i64) -> i64 {
    let mut p = p;
    let mut aa = a % modulo;
    let mut ret = 1;
    while p >= 1 {
        if p & 1 == 1 {
            ret *= aa;
            ret %= modulo;
        }
        p >>= 1;
        aa = aa * aa % modulo;
    }
    ret
}

fn build_dbg(a: i64, b: i64, from_index: i64, to_index: i64, keta: i64, modulo: i64) -> i64 {
    let mut first_num = a + from_index * b;
    let last_num = a + to_index * b;
    let keta = pow(10, keta, modulo);
    let mut ret = 0;
    while first_num <= last_num {
        ret *= keta;
        ret += first_num;
        ret %= modulo;
        first_num += b;
    }
    ret
}

fn build(a: i64, b: i64, from_index: i64, to_index: i64, keta: i64, modulo: i64) -> i64 {
    let first_num = (a % modulo + from_index % modulo * b % modulo) % modulo;
    let b = b % modulo;
    let pw = pow(10, keta, modulo);
    let sqsum = vec![
        vec![pw, 1, 0, 0],
        vec![0,  1, 0, 0],
        vec![1, 0,  1, 0],
        vec![0, 1,  0, 1]
    ];
    let mat1 = SquareMatrix::new(sqsum, modulo);
    let pmat = mat1.pow(to_index-from_index+1);

    let one = pmat.data[2][0] % modulo;
    let onetwo = pmat.data[2][1] % modulo;
    (one * first_num + onetwo * b) % modulo
}

fn main() {
    input! {
        l: i64, a: i64, b: i64, modulo: i64
    };

    // debug!(
    //     build_dbg(10000000000007, 1000000000000007, 100, 106, 18, 998244353),
    //     build(10000000000007, 1000000000000007, 100, 106, 18, 998244353)
    // );

    let mut indice_pairs = vec![];
    let mut keta = 1;
    let mut knum = 1;
    while keta < 1e18 as i64 {
        let (f, t) = indices(keta, keta*10-1, a, b);
        if f != -1 && f <= l-1 && f <= t {
            let t = min(t, l-1);
            indice_pairs.push((f, t, knum));
        }
        knum += 1;
        keta *= 10;
    }
    indice_pairs.reverse();
    // debug!(indice_pairs);

    let mut total = 0;
    let mut left_keta = 0;
    let mut ketamod = 1e12 as i64;
    let mut large10 = pow(10, ketamod, modulo);

    let mut lkmul = 1;
    for (f, t, kn) in indice_pairs {
        total += build(a, b, f, t, kn, modulo) * lkmul % modulo;
        total %= modulo;

        let sz = t-f+1;
        let large = (sz/ketamod)*kn;
        let small = (sz%ketamod)*kn;
        let lklk = pow(large10, large, modulo) * pow(10, small, modulo) % modulo;
        lkmul *= lklk;
        lkmul %= modulo;

    }
    println!("{}", total);
}

// 999999999999999999 1 1 100

