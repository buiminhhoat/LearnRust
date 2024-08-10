use std::{cmp::max, f32::consts::E, io::{self}, usize::MAX};

static MAXN: usize = 205;
static MOD: i64 = 1000000007;

struct DP {
    n: i64,
    dp: Vec<Vec<Vec<i32>>>,
    m: i64,
    x: i64
}

impl DP {
    fn DP(n: i64, m: i64, x: i64) -> DP {
        let mut dp = vec![vec![vec![-1; (n + 5) as usize]; (n + 5) as usize]; (m + 5) as usize];
        DP {
            n: n,
            dp: dp,
            m: m,
            x: x
        }
    }

    fn solve(&mut self, i: i64, open: i64, cnt: i64) -> i32 {
        if open > self.n || cnt > self.n {
            return 0;
        }
        if i > self.m {
            if open == 0 && cnt == self.n {
                return 1;
            }
            return 0;
        }

        if self.dp[i as usize][open as usize][cnt as usize] != -1 {
            return self.dp[i as usize][open as usize][cnt as usize];
        }

        let mut cur: i64 = 0;

        if i == self.x {
            cur += self.solve(i + 1, open + 1, cnt + 1) as i64;

            if open == 0 {
                cur += self.solve(i + 1, open, cnt + 1) as i64;
                cur %= MOD;
            }
            
            if open >= 1 {
                cur += self.solve(i + 1, open, cnt + 1) as i64;
                cur %= MOD;
            }
        }
        else {
            cur += self.solve(i + 1, open, cnt) as i64;
            cur %= MOD;

            cur += self.solve(i + 1, open + 1, cnt + 1) as i64;
            cur %= MOD;

            if open - 1 >= 0 {
                cur += self.solve(i + 1, open - 1, cnt) as i64;
                cur %= MOD;

                cur += self.solve(i + 1, open, cnt + 1) as i64;
                cur %= MOD;
            }
            
            if open == 0 {
                cur += self.solve(i + 1, open, cnt + 1) as i64;
                cur %= MOD;
            }
        }

        self.dp[i as usize][open as usize][cnt as usize] = cur as i32;
        return cur as i32;
    }
}

fn main() {
    let ints = read_ints();
    let n = ints[0];
    let m = ints[1];
    let x = ints[2];

    if n > m {
        println!("0");
        return;
    }

    let mut dp = DP::DP(n, m, x);
    
    let mut gt: i64 = 1;
    for i in 1 .. n + 1 {
        gt = (gt * i) % MOD;
    }
    println!("{}", (gt * dp.solve(1 as i64, 0 as i64, 0 as i64) as i64) % MOD);
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_ints() -> Vec<i64> {
    read_str().split_whitespace().map(|x| x.parse().unwrap()).collect()
}
