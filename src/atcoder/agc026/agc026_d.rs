// https://atcoder.jp/contests/agc026/tasks/agc026_d
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

const MOD: i64 = 1e9 as i64 + 7;

fn powmod(a: i64, p: i64, m: i64) -> i64 {
    let mut ret = 1i64;
    let mut aa = a;
    let mut pp = p;
    while pp >= 1 {
        if pp & 1 == 1 {
            ret *= aa;
            ret %= m;
        }
        aa = aa * aa % m;
        pp >>= 1;
    }
    ret
}

fn pow2(p: i64) -> i64 {
    powmod(2, p, MOD)
}

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };


    let mut aw = a.clone();
    aw.sort();

    let mut blocks = vec![];
    {
        let mut last = 0;
        for h in aw {
            if last < h {
                blocks.push(h - last);
            }
            last = h;
        }
    }
    let mut heights = vec![0; n];
    for i in 0..n {
        let mut cnt = 0;
        let mut sum = 0;
        for &b in &blocks {
            sum += b;
            cnt += 1;
            if sum == a[i] {
                heights[i] = cnt;
                break;
            }
        }
    }

    let hn = blocks.len();
    let mut dp = vec![vec![vec![0; 2]; hn+1]; n+1];
    dp[0][0][0] = 1;

    // println!("{:?}", blocks);
    // println!("{:?}", heights);

    for i in 0..n {
        for h in 0..hn+1 {
            for f in 0..2 {
                let base = dp[i][h][f];
                if base == 0 {
                    continue;
                }



                // [from, to]
                let mut fromh = 1;
                let mut toh = 0;

                let mut sameh = h;
                if i == 0 {
                    fromh = 1;
                    toh = heights[i];
                } else {
                    fromh = heights[i-1]+1;
                    toh = heights[i];
                    if (f == 0 && h > heights[i]) || (f == 1 && h >= heights[i]) {
                        sameh = 0;
                    }
                }

                // println!("{} {} {} {} / {} {} {}", i, h, f, base, fromh, toh, sameh);

                let prevways = if sameh == 0 { 2 } else { 1 };
                let mut totalh = 0;
                for hi in fromh..toh+1 {
                    totalh += blocks[hi-1];
                }

                if sameh == 0 {
                    for hi in fromh..toh+1 {
                        if hi >= 2 {
                            // between
                            dp[i+1][hi-1][1] += base * prevways % MOD * pow2(totalh-1) % MOD;
                            dp[i+1][hi-1][1] %= MOD;

                            // else
                            dp[i+1][hi][0] += base * prevways % MOD * (pow2(blocks[hi-1]-1) + MOD - 1) % MOD * pow2(totalh - blocks[hi-1]) % MOD;
                            dp[i+1][hi][0] %= MOD;
                        } else {
                            // else
                            dp[i+1][hi][0] += base % MOD * (pow2(blocks[hi-1]) + MOD - 2) % MOD * pow2(totalh - blocks[hi-1]) % MOD;
                            dp[i+1][hi][0] %= MOD;
                        }
                        totalh -= blocks[hi-1];
                    }

                    dp[i+1][0][0] += base * prevways % MOD;
                    dp[i+1][0][0] %= MOD;
                } else {
                    dp[i+1][h][f] += base * pow2(totalh);
                    dp[i+1][h][f] %= MOD;
                }
           }
        }
    }

    let mut total = 0;
    for h in 0..hn+1 {
        for f in 0..2 {
            total += dp[n][h][f];
            total %= MOD;
        }
    }
    println!("{}", total);
}
