#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

trait InputValue {
    fn parse(s: &str) -> Self;
}

fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

macro_rules! parse_single_value {
    ($($t:ty),*) => {
        $(
            impl InputValue for $t {
                fn parse(s: &str) -> $t { s.parse().unwrap() }
            }
        )*
	}
}
parse_single_value!(i32, i64, f32, f64, usize, String);

macro_rules! parse_tuple {
	($($t:ident),*) => {
		impl<$($t),*> InputValue for ($($t),*) where $($t: InputValue),* {
			fn parse(s: &str) -> ($($t),*) {
				let mut tokens = s.split_whitespace();
				let t = ($($t::parse(tokens.next().unwrap())),*);
				t
			}
		}
	}
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);

// ===

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    let n: usize = read();
    let mut dp = vec![vec![vec![vec![0; 2]; 2]; 2]; 62];

    dp[61][0][0][0] = 1;
    for i in (1..62).rev() {
        let on = (n & (1<<(i-1))) >= 1;
        for uflg in 0..2 {
            for vflg in 0..2 {
                for lend in 0..2 {
                    let base = dp[i][uflg][vflg][lend];
                    if base == 0 {
                        continue
                    }

                    let mut flg = vec![vec![vec![0; 2]; 2]; 2];
                    for a in 0..2 {
                        for b in 0..2 {
                            for borrow in 0..2 {
                                let should_lend = if a + b + borrow >= 2 { 1 } else { 0 };
                                if lend != should_lend {
                                    continue
                                }
                                let u = if a == b { 0 } else { 1 };
                                let v = (a + b + borrow) % 2;
                                flg[u][v][borrow] = 1;
                            }
                        }
                    }

                    for u in 0..2 {
                        for v in 0..2 {
                            for bo in 0..2 {
                                if uflg == 0 && !on && u == 1 {
                                    continue
                                }
                                if vflg == 0 && !on && v == 1 {
                                    continue
                                }
                                let tuflg = max(uflg, if on && u == 0 { 1 } else { 0 });
                                let tvflg = max(vflg, if on && v == 0 { 1 } else { 0 });
                                if flg[u][v][bo] == 0 {
                                    continue
                                }
                                dp[i-1][tuflg][tvflg][bo] += base;
                                dp[i-1][tuflg][tvflg][bo] %= MOD;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for uflg in 0..2 {
        for vflg in 0..2 {
            sum += dp[0][uflg][vflg][0];
        }
    }
    println!("{}", sum % MOD);
}