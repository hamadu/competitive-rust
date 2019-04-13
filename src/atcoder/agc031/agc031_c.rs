// https://atcoder.jp/contests/agc031/tasks/agc031_c
//
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::fmt::*;
use std::io::*;
use std::str::*;

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

fn main() {
    input! {
        n: usize, a: usize, b: usize
    };
    let mut visited = vec![false; n];
    let mut ans: Vec<usize> = vec![];

    let mut an = vec![0; n];
    let mut bn = vec![0; n];
    let mut diffcnt = 0;
    for i in 0..n {
        an[i] = (a >> i) & 1;
        bn[i] = (b >> i) & 1;
        diffcnt += if an[i] == bn[i] { 0 } else { 1 };
    }
    if diffcnt % 2 == 0 {
        println!("NO");
        return;
    }

    for i in 0..n {
        if an[i] != bn[i] {
            visited[i] = true;
            ans.push(an[i] << i);
            ans.push(bn[i] << i);
            break;
        }
    }
    for i in 0..n {
        if an[i] == bn[i] {
            let up = an[i] << i;
            let dw = (1 << i) - up;
            let mut nans = vec![];
            let k = ans.len() / 2;
            for j in 0..k {
                let l = ans[j * 2];
                let r = ans[j * 2 + 1];
                nans.push(l ^ up);
                nans.push(l ^ dw);
                nans.push(r ^ dw);
                nans.push(r ^ up);
            }
            ans = nans;
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if an[i] != bn[i] && an[j] != bn[j] && !visited[i] && !visited[j] {
                visited[i] = true;
                visited[j] = true;

                let fr = an[i] << i;
                let ba = (1 << i) - fr;

                let up = an[j] << j;
                let dw = (1 << j) - up;

                let k = ans.len() / 2;
                let mut nans = vec![];
                for ki in 0..k {
                    let l = ans[ki * 2];
                    let r = ans[ki * 2 + 1];

                    if ki == k - 1 {
                        nans.push(l ^ fr ^ up);
                        nans.push(l ^ ba ^ up);
                        nans.push(r ^ ba ^ up);
                        nans.push(r ^ fr ^ up);
                        nans.push(r ^ fr ^ dw);
                        nans.push(l ^ fr ^ dw);
                        nans.push(l ^ ba ^ dw);
                        nans.push(r ^ ba ^ dw);
                    } else {
                        nans.push(l ^ fr ^ up);
                        nans.push(l ^ ba ^ up);
                        nans.push(l ^ ba ^ dw);
                        nans.push(l ^ fr ^ dw);
                        nans.push(r ^ fr ^ dw);
                        nans.push(r ^ ba ^ dw);
                        nans.push(r ^ ba ^ up);
                        nans.push(r ^ fr ^ up);
                    }
                }
                ans = nans;
            }
        }
    }

    println!("YES");
    let n = ans.len();
    for i in 0..n {
        if i >= 1 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
}
