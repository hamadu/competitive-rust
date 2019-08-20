// https://atcoder.jp/contests/arc084/tasks/arc084_d
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
#[derive(Clone,Debug)]
struct BitVector {
    n: usize,
    size: usize,
    data: Vec<u64>
}

impl BitVector {
    fn new(n: usize) -> Self {
        let bsize = 8 * std::mem::size_of::<usize>();
        let size = (n+bsize-1)/bsize;
        BitVector {
            n: n,
            size: size,
            data: vec![0; size]
        }
    }

    fn get(&self, i: usize) -> bool {
        let bsize = 8 * std::mem::size_of::<usize>();
        self.data[i/bsize]>>(i&(bsize-1)) & 1 != 0
    }

    fn set(&mut self, i: usize, v: bool) {
        let bsize = 8 * std::mem::size_of::<usize>();
        let (bucket, index) = (i/bsize, i&(bsize-1));
        if v {
            self.data[bucket] |= 1<<index;
        } else {
            self.data[bucket] &= !(1<<index);
        }
    }

    fn msb(&self) -> Option<usize> {
        let bsize = 8 * std::mem::size_of::<usize>();
        for i in (0..self.size).rev() {
            if self.data[i] != 0 {
                return Some(bsize * i + bsize - 1 - self.data[i].leading_zeros() as usize);
            }
        }
        None
    }
}

impl<'a> std::ops::Shl<usize> for &'a BitVector {
    type Output = BitVector;

    fn shl(self, amount: usize) -> Self::Output {
        let mut new_vector = BitVector::new(self.n);
        let bsize = 8 * std::mem::size_of::<usize>();

        let diff = amount / bsize;
        let diff_mod = amount & (bsize - 1);
        if diff_mod == 0 {
            for i in 0..self.size-diff {
                new_vector.data[i+diff] = self.data[i];
            }
        } else {
            let lower_mask = (1<<(bsize-diff_mod))-1;
            let upper_mask = ((1<<diff_mod)-1)<<(bsize-diff_mod);
            for i in 0..self.size-diff {
                new_vector.data[i+diff] |= (self.data[i] & lower_mask)<<diff_mod;
                if i+diff+1 < self.size {
                    new_vector.data[i+diff+1] |= (self.data[i] & upper_mask)>>(bsize-diff_mod);
                }
            }
        }
        new_vector
    }
}

impl std::ops::BitAndAssign for BitVector {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..min(self.size, rhs.size) {
            self.data[i] &= rhs.data[i];
        }
    }
}

impl std::ops::BitOrAssign for BitVector {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..min(self.size, rhs.size) {
            self.data[i] |= rhs.data[i];
        }
    }
}

impl std::ops::BitXorAssign for BitVector {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..min(self.size, rhs.size) {
            self.data[i] ^= rhs.data[i];
        }
    }
}

// ===

fn to_bv(a: &Vec<char>) -> BitVector {
    assert!(a.len() <= 4000);
    let mut bv = BitVector::new(4000);
    for i in 0..a.len() {
        if a[i] == '1' {
            bv.set(a.len()-i-1, true);
        }
    }
    bv
}

fn gcd(mut a: BitVector, mut b: BitVector) -> BitVector {
    let (alen, blen) = match (a.msb(), b.msb()) {
        (None, _) => return b,
        (_, None) => return a,
        (Some(alen), Some(blen)) => (alen, blen)
    };
    if alen < blen {
        std::mem::swap(&mut a, &mut b);
    }
    let al = max(alen, blen);
    let bl = min(alen, blen);

    // al >= bl
    use std::ops::Shl;
    for i in 0..al-bl+1 {
        let d = al-bl-i;
        if a.get(al-i) {
            a ^= b.shl(d);
        }
    }
    gcd(a, b)
}

const MOD: i64 = 998244353;

fn solve(mut x: BitVector, g: BitVector, pow2: &Vec<i64>) -> i64 {
    let xl = x.msb().unwrap();
    let gl = g.msb().unwrap();

    // debug!(x, g, xl, gl);

    if xl < gl {
        return 1; // '0'
    }

    let mut ans = 0;
    for i in gl..xl+1 {
        if x.get(i) {
            ans += pow2[i-gl];
            ans %= MOD;
        }
    }

    let mx = x.clone();

    use std::ops::Shl;
    for i in (gl..xl+1).rev() {
        if x.get(i) {
            x ^= g.shl(i-gl);
        }
    }

    // debug!(x, mx, gl);

    let mut rx = mx.clone();
    rx ^= x;
    let mut less_or_eq = true;
    for i in (0..gl).rev() {
        if rx.get(i) != mx.get(i) {
            less_or_eq = mx.get(i);
            break;
        }
    }
    if less_or_eq {
        ans += 1;
    }
    ans % MOD
}

fn main() {
    input! {
        n: usize, x: chars,
        a: [chars; n]
    };

    let x = to_bv(&x);

    let mut g = to_bv(&a[0]);
    for i in 1..n {
        g = gcd(g, to_bv(&a[i]));
    }
    // debug!(g);

    let mut pow2 = vec![1; 4001];
    for i in 1..pow2.len() {
        pow2[i] = (pow2[i-1] * 2) % MOD;
    }
    let ans = solve(x, g, &pow2);

    println!("{}", ans);
}
