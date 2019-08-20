// https://atcoder.jp/contests/agc020/tasks/agc020_c
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

use std::ops::{Index,IndexMut};
use std::ops::{Shl,Shr,BitOrAssign};

struct BitVector {
    n: usize,
    size: usize,
    data: Vec<u64>
}

impl BitVector {
    fn new(n: usize) -> Self {
        let size = (n+63)/64;
        BitVector {
            n: n,
            size: size,
            data: vec![0; size]
        }
    }

    fn get(&self, i: usize) -> bool {
        self.data[i/64]>>(i%64) & 1 != 0
    }

    fn set(&mut self, i: usize) {
        self.data[i/64] |= 1<<(i%64);
    }
}

impl<'a> Shl<usize> for &'a BitVector {
    type Output = BitVector;

    fn shl(self, amount: usize) -> Self::Output {
        let mut new_vector = BitVector::new(self.n);

        let diff = amount / 64;
        let diff_mod = amount % 64;
        if diff_mod == 0 {
            for i in 0..self.size-diff {
                new_vector.data[i+diff] = self.data[i];
            }
        } else {
            let lower_mask = (1<<(64-diff_mod))-1;
            let upper_mask = ((1<<diff_mod)-1)<<(64-diff_mod);
            for i in 0..self.size-diff {
                new_vector.data[i+diff] |= (self.data[i] & lower_mask)<<diff_mod;
                if i+diff+1 < self.size {
                    new_vector.data[i+diff+1] |= (self.data[i] & upper_mask)>>(64-diff_mod);
                }
            }
        }
        new_vector
    }
}

impl BitOrAssign for BitVector {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..self.size {
            self.data[i] |= rhs.data[i];
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut v = BitVector::new(2000*2000+10);
    v.set(0);
    for i in 0..n {
        let nv = &v << a[i];
        v |= nv;
    }

    let mut kinds = vec![];
    for i in 0..v.n {
        if v.get(i) {
            kinds.push(i);
        }
    }

    let m = kinds.len();
    println!("{}", kinds[m/2]);
}
