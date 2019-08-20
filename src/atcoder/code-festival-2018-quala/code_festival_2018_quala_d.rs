// https://atcoder.jp/contests/code-festival-2018-quala/tasks/code_festival_2018_quala_d
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

const MOD: i64 = 1000000007;

fn main() {
    input! {
        d: i64, f: i64, t: i64, n: usize,
        x: [i64; n]
    };

    let mut x = x;
    x.insert(0, 0);
    x.push(d);

    let mut pw2 = vec![0; n+10];
    pw2[0] = 1;
    for i in 1..pw2.len() {
        pw2[i] = pw2[i-1] * 2 % MOD;
    }

    let mut goto = vec![0; n+1];
    let mut refill_free = vec![0; n+1];

    let mut gi = 0;
    let mut fi = 0;
    for i in 0..n+1 {
        while fi <= n+1 && x[fi] - x[i] <= f {
            goto[i] = fi;
            fi += 1;
        }
        fi -= 1;
        while gi <= n+1 && x[gi] - x[i] <= f - t {
            refill_free[i] = gi;
            gi += 1;
        }
        gi -= 1;
    }

    let mut dp = vec![0; n+1];
    dp[0] = 1;

    let mut wsum = vec![0; n+10];
    let mut addtd = 0;
    for i in 0..n+1 {
        addtd += wsum[i];
        addtd %= MOD;
        dp[i] += addtd;
        dp[i] %= MOD;
        let base = dp[i];
        let ff = refill_free[i] - i;
        let add = base * pw2[ff] % MOD;

        let from = refill_free[i]+1;
        let to = goto[i];
        if from <= to {
            wsum[from] += add;
            wsum[from] %= MOD;
            wsum[to+1] += MOD - add;
            wsum[to+1] %= MOD;
        }
    }

    let mut ans = 0;
    for i in 0..n+1 {
        if d-x[i] <= f {
            let ff = min(n, refill_free[i]) - i;
            ans += dp[i] * pw2[ff] % MOD;
            // debug!(i, dp[i], pw2[ff]);
            ans %= MOD;
        }
    }


    println!("{}", ans);
}
