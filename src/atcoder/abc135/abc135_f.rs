// https://atcoder.jp/contests/abc135/tasks/abc135_f
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

// (z[idx] := longest common prefix of ([idx,n), [0,n))
fn z(a: &Vec<char>) -> Vec<usize> {
    let n = a.len();
    let mut z = vec![0; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i > r {
            l = i;
            r = i;
            while r < n && a[r-l] == a[r] {
                r += 1;
            }
            z[i] = r - l;
            r -= 1;
        } else {
            let k = i-l;
            if z[k] < r-i+1 {
                z[i] = z[k];
            } else {
                l = i;
                while r < n && a[r-l] == a[r] {
                    r += 1;
                }
                z[i] = r-l;
                r -= 1;
            }
        }
    }
    z
}

fn main() {
    input! {
        s: chars, t: chars
    };
    let tl = t.len();

    let sl = (2 * tl + s.len() - 1) / s.len() * s.len() * 2;
    let mut ss = vec![' '; sl];
    for i in 0..sl {
        ss[i] = s[i%s.len()];
    }

    let mut zs = vec![];
    zs.append(&mut t.clone());
    zs.push('@');
    zs.append(&mut ss.clone());
    // println!("{:?}", zs);

    let zi = z(&zs);
    let mut next = vec![false; sl];
    let mut prev = vec![false; sl];
    let mut nodes = 0;
    for i in tl+1..zi.len() {
        let si = i-tl-1;
        if zi[i] == tl || (si >= sl/2 && zi[i-sl/2] == tl) {
            next[si] = true;
            prev[(si+tl)%sl] = true;
            nodes += 1;
        }
    }

    let mut ans = 0;
    let mut vlnd = 0;
    for i in 0..sl {
        if !prev[i] && next[i] {
            let mut j = i;
            let mut cnt = 0;
            while next[j] {
                j = (j + tl) % sl;
                cnt += 1;
                vlnd += 1;
            }
            ans = max(ans, cnt);
        }
    }

    if vlnd < nodes {
        println!("-1")
    } else {
        println!("{}", ans);
    }
}
