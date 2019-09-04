// https://atcoder.jp/contests/abc137/tasks/abc137_f
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

// ===

/// Polynomial with mod prime number.
/// Example usage:
///
/// ```
/// let poly = PolynomialModP::new(5);
/// let l1 = pe.linear(1, 2);           // x+2
/// let l2 = pe.linear(3, 4);           // 3x+4
/// assert_eq!(l1+l2, pe.linear(4, 1)); // 4x+6≡4x+1 (mod 5)
///
/// let l3 = pe.generate(vec![2, 0, 1]);                  // 2+x^2
/// let l4 = pe.generate(vec![1, 1, 2, 1]);               // 1+x+2x^2+x^3
/// assert_eq!(l3*l4, pe.generate(vec![2, 3, 0, 3, 2]));  // 2+3x+3x^3+2x^4 (mod 5)
/// ```
struct PolynomialModP {
    p: usize,
    fact: Vec<usize>,
    invfact: Vec<usize>
}

impl PolynomialModP {
    fn new(p: usize) -> Self {
        let mut fact = vec![0; p];
        fact[0] = 1;
        for i in 1..p {
            fact[i] = fact[i-1] * i % p;
        }
        let mut invfact = vec![0; p];
        invfact[p-1] = Polynomial::inv(fact[p-1], p);
        for i in (1..p).rev() {
            invfact[i-1] = invfact[i] * i % p;
        }
        // TODO: consider asserting given p is actually a prime?
        PolynomialModP { p: p, fact: fact, invfact: invfact }
    }

    fn generate(&self, mut a: Vec<usize>) -> Polynomial {
        for i in 0..a.len() {
            a[i] %= self.p;
        }
        Polynomial { p: self.p, a: a }
    }

    /// Generates 0.
    fn zero(&self) -> Polynomial {
        Polynomial { p: self.p, a: vec![0] }
    }

    /// Generates 1.
    fn one(&self) -> Polynomial {
        Polynomial { p: self.p, a: vec![1] }
    }

    /// Generates ax + b.
    fn linear(&self, a: usize, b: usize) -> Polynomial {
        Polynomial { p: self.p, a: vec![b, a] }
    }

    // Generates (ax+b)^w.
    fn linear_pow(&self, a: usize, b: usize, mut w: usize) -> Polynomial {
        if w >= self.p {
            w -= 1;
            w %= self.p - 1;
            w += 1;
        }
        let mut v = vec![0; self.p];
        let mut ap = 1;
        for i in 0..w+1 {
            let comb = self.fact[w] * self.invfact[i] % self.p * self.invfact[w-i] % self.p;
            v[i] = ap % self.p * comb % self.p;
            ap = (ap * a) % self.p;
        }
        let mut bp = 1;
        for i in (0..w+1).rev() {
            v[i] = (v[i] * bp) % self.p;
            bp = (bp * b) % self.p;
        }

        Polynomial { p: self.p, a: v }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Polynomial {
    p: usize,
    a: Vec<usize>
}

impl Polynomial {
    fn get(&self, index: usize) -> usize {
        assert!(index < self.p);
        *self.a.get(index).unwrap_or(&0)
    }

    fn powmod(a: usize, p: usize, m: usize) -> usize {
        let mut ret = 1usize;
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

    fn inv(a: usize, m: usize) -> usize {
        Self::powmod(a, m-2, m)
    }
}


impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let w = max(self.a.len(), rhs.a.len());
        let mut v = vec![0; w];
        for i in 0..w {
            v[i] = (*self.a.get(i).unwrap_or(&0) + *rhs.a.get(i).unwrap_or(&0)) % self.p;
        }
        Polynomial { p: self.p, a: v }
    }
}

impl std::ops::AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.p, rhs.p);
        let w = max(self.a.len(), rhs.a.len());
        self.a.resize(w, 0);
        for i in 0..rhs.a.len() {
            self.a[i] = (self.a[i] + rhs.a[i]) % self.p;
        }
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let w = max(self.a.len(), rhs.a.len());
        let mut v = vec![0; w];
        for i in 0..w {
            v[i] = (*self.a.get(i).unwrap_or(&0) + self.p - *rhs.a.get(i).unwrap_or(&0)) % self.p;
        }
        Polynomial { p: self.p, a: v }
    }
}

impl std::ops::SubAssign for Polynomial {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.p, rhs.p);
        let w = max(self.a.len(), rhs.a.len());
        self.a.resize(w, 0);
        for i in 0..rhs.a.len() {
            self.a[i] = (self.a[i] + self.p - rhs.a[i]) % self.p;
        }
    }
}

impl std::ops::MulAssign<usize> for Polynomial {
    fn mul_assign(&mut self, rhs: usize) {
        for i in 0..self.p {
            self.a[i] = (self.a[i] * rhs) % self.p;
        }
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.p, rhs.p);
        let mut to = vec![0; min(self.a.len() + rhs.a.len(), self.p)];
        for i in 0..self.a.len() {
            for j in 0..rhs.a.len() {
                let add = (self.a[i] * rhs.a[j]) % self.p;
                let k = i+j;
                if k >= self.p {
                    to[k-(self.p-1)] += add;
                } else {
                    to[k] += add;
                }
            }
        }
        for i in 0..to.len() {
            to[i] %= self.p;
        }
        Polynomial { p: self.p, a: to }
    }
}

impl std::ops::MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.p, rhs.p);
        let mut to = vec![0; min(self.a.len() + rhs.a.len(), self.p)];
        for i in 0..self.a.len() {
            for j in 0..rhs.a.len() {
                let add = self.a[i] * rhs.a[j];
                let k = i+j;
                if k >= self.p {
                    to[k-(self.p-1)] += add;
                } else {
                    to[k] += add;
                }
            }
        }
        // TODO: consider just moving from to?
        self.a.resize(to.len(), 0);
        for i in 0..to.len() {
            self.a[i] = to[i] % self.p;
        }
    }
}

impl std::ops::DivAssign<usize> for Polynomial {
    fn div_assign(&mut self, rhs: usize) {
        let inv = Polynomial::inv(rhs, self.p);
        for i in 0..self.p {
            self.a[i] = (self.a[i] * inv) % self.p;
        }
    }
}

// ===

fn main() {
    input! {
        p: usize,
        a: [usize; p]
    };

    let poly = PolynomialModP::new(p);
    let mut ans = poly.zero();

    // tmp = x(x-1)...(x-(p-1)) ≡ (x^p-x) mod p
    let mut tmp = vec![0; p+1];
    for i in 0..p {
        if a[i] == 0 {
            continue;
        }
        let mut div = 1;
        for j in 0..p {
            if i == j {
                continue;
            }
            div *= (i+p-j)%p;
            div %= p;
        }

        // divide (x^p-x) by (x-i)
        let mut all = poly.generate(vec![0; p]);
        let inv = Polynomial::inv(div, p);
        if i == 0 {
            // divide by x
            all.a[p-1] = 1;
            all.a[0] = p-1;
        } else {
            // divide by (x-i)
            for j in 0..p+1 {
                tmp[j] = 0;
            }
            tmp[p] = 1;
            tmp[1] = p-1;
            for j in (0..p).rev() {
                all.a[j] = tmp[j+1];
                tmp[j] = (tmp[j] + i * tmp[j+1]) % p;
                tmp[j] %= p;
            }
        }

        all *= inv;
        ans += all;
    }

    let le = (0..p).map(|i| ans.get(i).to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", le);
}
