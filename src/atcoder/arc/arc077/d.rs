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

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    let mut n: usize = read();
    let a: Vec<usize> = readnc();
    n += 1;

    let mut z = vec![-1; n+1];
    let mut fr = 0;
    let mut to = 0;#![allow(unused_imports, unused_variables, dead_code)]
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

    const MOD: i64 = 1e9 as i64 + 7;

    fn main() {
        let mut n: usize = read();
        let a: Vec<usize> = readnc();
        n += 1;

        let mut z = vec![-1; n+1];
        let mut fr = 0;
        let mut to = 0;
        for i in 0..n {
            if z[a[i]] != -1 {
                fr = z[a[i]] as usize;
                to = i;
            } else {
                z[a[i]] = i as i32;
            }
        }

        let comb = Combination::new(n+100, MOD);
        let outer = fr + n - 1 - to;
        for c in 1..n+1 {
            let mut sum = 0;
            // 1: zero
            sum += comb.combination(n-2, c);

            // 1: one
            sum += comb.combination(n-2, c-1) * 2;
            sum += MOD - comb.combination(outer, c-1);

            if c >= 2 {
                sum += comb.combination(n-2, c-2);
            }

            println!("{}", sum % MOD);
        }
    }
    for i in 0..n {
        if z[a[i]] != -1 {
            fr = z[a[i]] as usize;
            to = i;
        } else {
            z[a[i]] = i as i32;
        }
    }

    let comb = Combination::new(n+100, MOD);
    let outer = fr + n - 1 - to;
    for c in 1..n+1 {
        let mut sum = 0;
        // 1: zero
        sum += comb.combination(n-2, c);

        // 1: one
        sum += comb.combination(n-2, c-1) * 2;
        sum += MOD - comb.combination(outer, c-1);

        if c >= 2 {
            sum += comb.combination(n-2, c-2);
        }

        println!("{}", sum % MOD);
    }
}