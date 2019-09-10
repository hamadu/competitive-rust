// https://atcoder.jp/contests/abc132/tasks/abc132_f
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
        if $t {
            $a
        } else {
            $b
        }
    };
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

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        n: i64, k: usize
    };
    if n == 1 {
        println!("1");
        return;
    }

    let mut bucket_low = vec![];
    let mut bucket_high = vec![];
    for p in 1..n {
        if p*p <= n {
            bucket_low.push(vec![p, p+1]);
            if p*p < n {
                let low = n/(p+1);
                let high = n/p;
                bucket_high.push(vec![low+1, high+1, p]);
            }
        } else {
            break;
        }
    }

    bucket_high.reverse();
    if bucket_high.len() >= 1 && bucket_low.len() >= 1 {
        if bucket_high[0][0] == bucket_low[bucket_low.len()-1][0] && bucket_high[0][1] == bucket_low[bucket_low.len()-1][1] {
            bucket_high.remove(0);
        }
    }

    let bl = bucket_low.len();
    bucket_low.append(&mut bucket_high);
    let mut bucket = bucket_low;
    for i in 0..bl {
        let w = bucket.len()-i;
        bucket[i].push(w as i64);
    }


    // debug!(bucket);

    let n = bucket.len();
    let mut dp = dvec!(0; 2, n+1);
    dp[0][0] = 1;
    dp[0][n] = MOD-1;
    for w in 0..k {
        let fr = w % 2;
        let to = 1 - fr;
        fill!(dp[to], 0);
        for i in 0..n {
            dp[fr][i+1] += dp[fr][i];
            dp[fr][i+1] %= MOD;
        }
        if w == k-1 {
            break;
        }
        for i in 0..n {
            let base = dp[fr][i] * (bucket[i][1] - bucket[i][0]) % MOD;
            dp[to][0] += base;
            dp[to][0] %= MOD;
            dp[to][bucket[i][2] as usize] += MOD-base;
            dp[to][bucket[i][2] as usize] %= MOD;
        }
    }
    let mut total = 0;
    for i in 0..n {
        let base = dp[(k+1)%2][i] * (bucket[i][1] - bucket[i][0]) % MOD;
        total += base;
    }
    println!("{}", total % MOD);
}
