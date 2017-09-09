#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
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

const MOD: i64 = 998244353;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

struct Vector {
    dx: i32,
    dy: i32
}

impl Vector {
    fn new(p0: &Point, p1: &Point) -> Vector {
        Vector { dx: p1.x - p0.x, dy: p1.y - p0.y }
    }
}

fn cross(v1: &Vector, v2: &Vector) -> i32 {
    v1.dx * v2.dy - v1.dy * v2.dx
}

fn lined(p0: &Point, p1: &Point, p2: &Point) -> bool {
    let v1 = Vector::new(p0, p1);
    let v2 = Vector::new(p0, p2);
    cross(&v1, &v2) == 0
}


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

fn inv(a: i64, m: i64) -> i64 {
    powmod(a, m-2, m)
}

struct Combination {
    fact: Vec<i64>,
    invfact: Vec<i64>,
    modulo: i64
}

impl Combination {
    fn new(n: usize, modulo: i64) -> Self {
        let mut fact: Vec<i64> = vec![0; n];
        let mut invfact: Vec<i64> = vec![0; n];
        fact[0] = 1;
        for i in 1..n {
            fact[i] = fact[i-1] * i as i64 % modulo;
        }
        invfact[n-1] = inv(fact[n-1], modulo);
        for i in (0..n-1).rev() {
            invfact[i] = (invfact[i+1] * (i+1) as i64) % modulo;
        }

        Combination { fact: fact, invfact: invfact, modulo: modulo }
    }

    fn combination(&self, n: usize, k: usize) -> i64 {
        if n < k {
            return 0;
        }
        self.fact[n] * self.invfact[n-k] % self.modulo * self.invfact[k] % self.modulo
    }
}

fn main() {
    let n: usize = read();
    let points: Vec<(i32, i32)> = readn(n);
    let points: Vec<Point> = points.into_iter().map(|(a, b)| Point {x: a, y: b }).collect();
    let mut ans: i64 = 0;

    let comb = Combination::new(n+10, MOD);
    for i in 3..n+1 {
        ans += comb.combination(n, i);
        ans %= MOD;
    }

    let mut done: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for i in 0..n {
        for j in i+1..n {
            let ref a = points[i];
            let ref b = points[j];
            if done[i][j] {
                continue;
            }

            let mut online: Vec<usize> = vec![];
            for k in 0..n {
                if lined(&a, &b, &points[k]) {
                    online.push(k);
                }
            }
            let same = online.len();
            for nk in 3..same+1 {
                ans += MOD;
                ans -= comb.combination(same, nk);
                ans %= MOD;
            }
            for &x in &online {
                for &y in &online {
                    done[x][y] = true;
                }
            }
        }
    }

    println!("{}", ans);
}
