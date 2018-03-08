#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;

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

fn generate_primes(upto: usize) -> (Vec<i64>, Vec<bool>) {
    let mut is_prime = vec![true; upto+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..upto+1 {
        if is_prime[i] {
            let mut ii = i * i;
            while ii <= upto {
                is_prime[ii] = false;
                ii += i;
            }
        }
    }

    let mut primes = vec![];
    for i in 0..upto+1 {
        if is_prime[i] {
            primes.push(i as i64);
        }
    }
    (primes, is_prime)
}

fn insert(target: i64, anti: i64, anti_map: &mut HashMap<i64,i64>, count_map: &mut HashMap<i64, usize>) {
    anti_map.insert(anti, target);
    anti_map.insert(target, anti);
    match count_map.get(&target) {
        Some(&n) => count_map.insert(target, n+1),
        _        => count_map.insert(target, 1),
    };
}


fn main() {
    let n: usize = read();
    let a: Vec<i64> = readn(n);
    // let a: Vec<i64> = vec![9999999967; n];

    let (primes, is_prime) = generate_primes(100010);
    let pn = primes.len();

    let mut anti_map = HashMap::new();
    let mut count_map: HashMap<i64, usize> = HashMap::new();

    let mut ans = 0;
    let mut has_cube = false;
    for i in 0..n {
        let mut aa = a[i];
        let mut normalized = 1;
        let mut anti = 1;
        let mut is_cube = true;
        for pi in 0..pn {
            let p = primes[pi];
            if aa < p || p > 3200 {
                break
            }

            let mut cnt = 0;
            while aa % p == 0 {
                aa /= p;
                cnt += 1;
            }
            if cnt % 3 == 1 {
                normalized *= p;
                anti *= p;
                anti *= p;
                is_cube = false;
            } else if cnt % 3 == 2 {
                normalized *= p;
                normalized *= p;
                anti *= p;
                is_cube = false;
            }
        }

        let mut cango = false;
        if aa >= 2 {
            is_cube = false;
            if aa < is_prime.len() as i64 && is_prime[aa as usize] {
                normalized *= aa;
                anti *= aa;
                anti *= aa;
                cango = true;
            } else {
                let leftp = (aa as f64).sqrt() as i64;
                if leftp * leftp == aa {
                    normalized *= aa;
                    anti *= leftp;
                    cango = true;
                }
            }
        } else {
            cango = true;
        }

        if !is_cube {
            if cango {
                insert(normalized, anti, &mut anti_map, &mut count_map);
            } else {
                ans += 1;
            }
        }
        has_cube |= is_cube;
    }

    let mut processed = HashSet::new();
    for (num, &v0) in count_map.iter() {
        let anti = anti_map.get(&num).unwrap();
        if processed.contains(num) || processed.contains(anti) {
            continue
        }
        let zero = &0;
        let v1 = *count_map.get(&anti).unwrap_or(zero);
        ans += max(v0, v1);
        processed.insert(num);
        processed.insert(anti);
    }
    if has_cube {
        ans += 1;
    }
    println!("{}", ans);
}