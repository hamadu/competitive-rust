// https://atcoder.jp/contests/arc096/tasks/arc096_c
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

    ($iter:expr, [ next / $t:tt ]) => {
        {
            let len = read_value!($iter, usize);
            (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        }
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

fn grouping(n: usize, MOD: i64) -> Vec<Vec<i64>> {
    let mut dp = dvec!(0; n+1, n+1);
    dp[0][0] = 1;
    for i in 1..n+1 {
        for j in 1..i+1 {
            dp[i][j] = (dp[i-1][j-1] + (j as i64 * dp[i-1][j]) % MOD) % MOD + dp[i-1][j] % MOD;
        }
        dp[i][0] = 1;
    }
    dp
}

fn main() {
    input! {
        n: usize, MOD: i64
    };

    let mut comb = dvec!(0; n+1, n+1);
    for i in 0..n+1 {
        comb[i][0] = 1;
        comb[i][i] = 1;
        for j in 1..i {
            comb[i][j] = (comb[i-1][j-1] + comb[i-1][j]) % MOD;
        }
    }

    let mut pow2 = vec![0; n+1];
    pow2[0] = 1;
    for i in 1..n+1 {
        pow2[i] = pow2[i-1] * 2 % MOD;
    }

    let mut any2 = vec![0; n+1];
    any2[0] = 2;
    for i in 1..n+1 {
        any2[i] = (any2[i-1] * any2[i-1]) % MOD;
    }

    let mut st2 = grouping(n, MOD);
    let mut ans = any2[n];

    for i in 1..n+1 {
        let mut total = 0;
        let mut select = comb[n][i];

        let mut pwni = 1;
        for g in 0..i+1 {
            let split = st2[i][g];
            total += split * pwni % MOD;
            total %= MOD;

            pwni *= pow2[n-i];
            pwni %= MOD;
        }

        total *= any2[n-i];
        total %= MOD;
        total *= select;
        total %= MOD;

        if i % 2 == 1 {
            ans += MOD - total;
        } else {
            ans += total;
        }
        ans %= MOD;
    }

    println!("{}", ans);
}
