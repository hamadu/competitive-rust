#![allow(unused_imports)]
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

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
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
parse_tuple!(A, B, C, D);
parse_tuple!(A, B, C, D, E);

// ===

const INF: usize = 1000000;

fn main() {
    let (n, k): (usize, usize) = read();
    let s: Vec<String> = readn(n);
    let s: Vec<Vec<u8>> = s.into_iter().map(|s| s.into_bytes()).collect();

    let mut can_make = vec![vec![false; k+1]; n+1];
    can_make[n][k] = true;

    for i in (0..n).rev() {
        let f = s[i].len();
        for j in 0..k+1 {
            if can_make[i+1][j] {
                if j >= f {
                    can_make[i][j-f] = true;
                }
                can_make[i][j] = true;
            }
        }
    }


    let mut last: Vec<u8> = vec![];
    for i in 1..n+1 {
        // println!("===");
        // println!("{:?}", can_make[i-1]);
        // println!("{:?}", can_make[i]);

        let f = s[i-1].len();
        let mut cmp = s[i-1].clone();
        cmp.append(&mut last.clone());
        let z = z(&cmp);

        let mut best_size = INF-1;
        for j in f..k+1 {
            if can_make[i][j] && can_make[i-1][j-f] {
                //let rst = compare_naive(&z, &last, &s[i-1], best_size, j-f);
                let rst = compare(&z, &last, &s[i-1], best_size, j-f);
                if rst >= 0 {
                    best_size = j-f;
                }
            }
        }

        if best_size < INF-1 {
            let mut next = vec![];
            for i in 0..best_size {
                next.push(last[i]);
            }
            for &c in &s[i-1] {
                next.push(c);
            }

            // last[0..x]
            // last[0..y] + s  のひかく

            let nl = next.len();
            for j in 0..k+1 {
                if !can_make[i][j] {
                    continue;
                }
                if j > nl {
                    can_make[i][j] = false;
                } else if j < nl {
                    let lastn = last.len();
                    let mut best = INF*2;

                    if can_make[i-1][j] && j <= lastn {
                        best = INF + j;
                    }

                    if j >= f && can_make[i-1][j-f] && j-f <= lastn {
                        if best == INF*2 || compare(&z, &last, &s[i-1], best, j-f) >= 1 {
                            best = j-f;
                        }
                    }

                    if best == INF*2 || compare(&z, &last, &s[i-1], best, best_size) == 1 {
                        can_make[i][j] = false;
                    }

                    // if next < z {
                    //     if next.len() >= z.len() || next[..] < z[0..next.len()] {
                    //         can_make[i][j] = false;
                    //     }
                    // }
                }
            }
            last = next;
        } else {
            for j in 0..k+1 {
                can_make[i][j] &= can_make[i-1][j];
            }
        }
    }

    for i in 0..k {
        print!("{}", last[i] as char);
    }
    println!("");
}

fn compare_u8(a: &Vec<u8>, b: &Vec<u8>) -> i32 {
    let an = a.len();
    let bn = b.len();
    for i in 0..min(an, bn) {
        if a[i] < b[i] { 
            return -1;
        } else if a[i] > b[i] {
            return 1;
        }
    }
    return 1;
}

fn compare_naive(z: &Vec<usize>, last: &Vec<u8>, next: &Vec<u8>, a: usize, b: usize) -> i32 {
    if b > last.len() {
        return -1;
    }
    if a == INF-1 {
        // last vs last[0,b] + next
        let l: Vec<u8> = last[..].iter().cloned().collect();
        let mut r: Vec<u8> = last[0..b].iter().cloned().collect();
        r.append(&mut next.clone());
        return compare_u8(&l, &r);
    } else {
        // last[0,a] + next vs 
        let mut l: Vec<u8> = last[0..a].iter().cloned().collect();
        let mut r: Vec<u8> = last[0..b].iter().cloned().collect();
        l.append(&mut next.clone());
        r.append(&mut next.clone());
        return compare_u8(&l, &r);
    }
}

// z := next + last
fn compare(z: &Vec<usize>, last: &Vec<u8>, next: &Vec<u8>, a: usize, b: usize) -> i32 {
    if b > last.len() {
        return -1;
    }
    let n = z.len();
    let nl = next.len();
    if a >= INF {
        // last[0, a-INF] vs last[0,b] + next
        let a = a - INF;
        let aidx = nl+a;
        let bidx = nl+b;
        if aidx <= bidx {
            return 0;
        } else {
            // println!("{} - {} {} {}", bidx, z.len(), next.len(), last.len());
            let zv = z[bidx];
            if zv >= a-b {
                return 0;
            }
            // println!("{} {} {}", b+zv, a, last.len());
            return if next[zv] < last[b+zv] { 1 } else { -1 };
        }
    } else if a == INF-1 {
        // last vs last[0,b] + next
        let idx = nl + b;
        if idx == z.len() {
            return 0;
        }
        if b + z[idx] >= last.len() {
            return 0;
        }
        if z[idx] >= next.len() {
            return -1;
        }
        return if next[z[idx]] < last[b+z[idx]] { 1 } else { -1 };
    }
    if a == b {
        return 0;
    }
    let diff = b-a;
    if z[nl+a] >= diff {
        // next[diff...] vs next[0...]
        if z[diff] + diff >= nl {
            return 0;
        }
        return if next[z[diff]] < next[diff+z[diff]] { 1 } else { -1 };

        // if a + next.len() <= b {
        //     return 0;
        // }
        // let l = a + next.len() - b;
        // if z[next.len()-l] >= l {
        //     return 0;
        // }
        // let zn = z[next.len()-l];
        // return if next[zn] < next[next.len()-l+zn] { 1 } else { -1 } ;
    } else {
        let zn = z[nl+a];
        if zn >= nl {
            return 1;
        }
        return if last[a+zn] < next[zn] { 1 } else { -1 };
    }
}

// (z[idx] := longest common prefix of ([idx,n), [0,n))
fn z(a: &Vec<u8>) -> Vec<usize> {
    let n = a.len();
    let mut z = vec![0; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i > r {
            l = i;
            r = i;
            while r < n && a[r-l] == a[r] {
                r += 1;
            }
            z[i] = r - l;
            r -= 1;
        } else {
            let k = i-l;
            if z[k] < r-i+1 {
                z[i] = z[k];
            } else {
                l = i;
                while r < n && a[r-l] == a[r] {
                    r += 1;
                }
                z[i] = r-l;
                r -= 1;
            }
        }
    }
    z
}
