// https://atcoder.jp/contests/agc020/tasks/agc020_d
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


fn isok(a: usize, b: usize, d: usize, g: usize) -> bool {
    if a < d * g || b < g - 1 {
        return false;
    }
    let ta = a - d * g;
    let tb = b - (g - 1);
    (tb + ta) / (ta + 1) <= d
}

fn isa(ptn: &Vec<(usize, usize, usize)>, x: usize) -> bool {
    let mut x = x;
    for &(a, b, cur) in ptn {
        if (a + b) * cur <= x {
            x -= (a + b) * cur;
        } else {
            return x % (a + b) < a;
        }

    }
    assert!(false);
    false
}


fn solve(a: usize, b: usize, ll: usize, rr: usize, flip: bool) -> String {
    let mut ans = String::new();

    assert!(a >= b);
    let d = (a + b) / (b + 1);

    // (a..ab)
    let mut aabaa = 0;
    {
        let mut lw = 0;
        let mut hi = a+1;
        while hi - lw > 1 {
            let md = (hi + lw) / 2;
            if isok(a, b, d, md) {
                lw = md;
            } else {
                hi = md;
            }
        }
        aabaa = lw;
    }

    let mut ptn = vec![];
    if aabaa >= 2 {
        ptn.push((d, 1, aabaa-1));
    }
    ptn.push((d, 0, 1));

    // println!("{} {} {} {} ", a, b, aabaa, d);

    let la = a - aabaa * d;
    let lb = b - (aabaa - 1);

    let mut tailcur = min(lb / d, la);

    let ma = la - tailcur;
    let mb = lb - tailcur * d;
    if ma >= 1 && mb >= 1 {
        ptn.push((0, 1, 1));
        ptn.push((ma, 0, 1));
        if mb >= 2 {
            ptn.push((0, mb-1, 1));
        }
    } else if ma >= 1 {

        if tailcur >= 1 {
            ptn.push((0, 1, 1));
            ptn.push((ma, 0, 1));
            ptn.push((0, d-1, 1));
            tailcur -= 1;
        } else {
            assert!(false);
        }
    } else if mb >= 1 {
        ptn.push((0, mb, 1));
    }
    ptn.push((1, d, tailcur));

    if !flip {
        for x in ll-1..rr {
            if isa(&ptn, x) {
                ans += "A";
            } else {
                ans += "B";
            }
        }
    } else {
        for x in (ll-1..rr).rev() {
            if isa(&ptn, x) {
                ans += "B";
            } else {
                ans += "A";
            }
        }
    }
   ans
}

fn main() {
    input! {
        q: usize,
        queries: [(usize, usize, usize, usize); q]
    };

    for q in queries {
        if q.0 >= q.1 {
            println!("{}", solve(q.0, q.1, q.2, q.3, false));
        } else {
            let all = q.0+q.1;
            println!("{}", solve(q.1, q.0, all+1-q.3, all+1-q.2, true));
        }
    }
}
