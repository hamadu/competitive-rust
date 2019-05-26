// https://atcoder.jp/contests/tenka1-2019/tasks/tenka1_2019_f
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
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const MOD: u64 = 998244353;

fn main() {
    input! {
        n: usize, x: usize
    };

    let mut comb = vec![vec![0; 3010]; 3010];
    for i in 0..comb.len() {
        comb[i][0] = 1;
        comb[i][i] = 1;
        for j in 1..i {
            let u = comb[i - 1][j - 1];
            let v = comb[i - 1][j];
            comb[i][j] = (u + v) % MOD;
        }
    }

    let mut ans = 0;
    for sum in 0..x {
        for one in 0..sum + 1 {
            let left = sum - one;
            if left % 2 == 1 {
                continue;
            }
            let two = left / 2;
            if one + two > n {
                continue;
            }
            ans += comb[n][one] * comb[n - one][two] % MOD;
            ans %= MOD;
        }
    }

    for sum in x + 1..2 * n + 1 {
        for one in 1..n + 1 {
            if sum < one {
                continue;
            }
            let two = (sum - one) / 2;
            if one + two * 2 != sum {
                continue;
            }
            if one + two > n {
                continue;
            }
            let prefix = (sum - x) / 2 + 1;
            if (sum - x) % 2 == 0 {
                continue;
            }
            if prefix * 2 > two {
                continue;
            }
            let total = two + one - prefix * 2;
            ans += comb[n][one + two] * comb[total][one] % MOD;
            ans %= MOD;
        }
        if sum % 2 == 0 && x % 2 != 0 {
            ans += comb[n][sum / 2];
            ans %= MOD;
        }
    }

    println!("{}", ans);
}
