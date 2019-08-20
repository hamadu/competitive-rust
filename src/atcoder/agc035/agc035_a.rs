// https://atcoder.jp/contests/agc035/tasks/agc035_a
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

#[allow(unused_macros)]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
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

fn isok(n: usize, map: HashMap<i32, usize>) -> bool {
    let mut values = vec![];
    let mut keys = vec![];
    for &k in map.keys() {
        keys.push(k);
        values.push(*map.get(&k).unwrap());
    }
    if values.len() == 1 {
        return keys[0] == 0;
    }
    if n % 3 != 0 {
        return false;
    }
    if values.len() >= 4 {
        return false;
    }
    let n3 = n / 3;
    if values.len() == 2 {
        if values[0] == n3 * 2 && values[1] == n3 {
            return keys[1] == 0;
        } else if values[1] == n3 * 2 && values[0] == n3 {
            return keys[0] == 0;
        }
    } else {
        // 3
        if keys[0] ^ keys[1] == keys[2] {
            return values[0] == n3 && values[1] == n3 && values[2] == n3;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let mut a = a;
    let mut map = HashMap::new();
    for i in 0..n {
        let &c = map.get(&a[i]).unwrap_or(&0);
        map.insert(a[i], c+1);
    }

    if isok(n, map) {
        println!("Yes");
    } else {
        println!("No");
    }
}
