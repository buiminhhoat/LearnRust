use std::{cmp::max, f32::consts::E, io::{self}, usize::MAX};

static MAXN: usize = 205;
static MOD: i64 = 1000000007;

struct DP {
    n: i64,
    a: Vec<i64>,
    dp: Vec<Vec<Vec<i32>>>,
    k: i64
}

impl DP {
    fn DP(n: i64, a: &[i64], k: i64) -> DP {
        let mut dp = vec![vec![vec![-1; 1005]; MAXN]; MAXN];
        DP {
            n: n,
            a: a.to_vec(),
            dp: dp,
            k: k
        }
    }

    fn solve(&mut self, i: i64, open: i64, k: i64) -> i32 {
        if k < 0 {
            return 0;
        }
        if i == self.n + 1 {
            if open == 0 && k == 0 {
                return 1;
            }
            return 0;
        }
        if self.dp[i as usize][open as usize][k as usize] != -1 {
            return self.dp[i as usize][open as usize][k as usize];
        }
        let mut cur: i64 = 0;

        let new_k = k - open * (self.a[i as usize] - self.a[(i - 1) as usize]);

        cur += self.solve(i + 1, open, new_k) as i64;
        cur %= MOD;

        cur += open * self.solve(i + 1, open, new_k) as i64;
        cur %= MOD;

        cur += self.solve(i + 1, open + 1, new_k) as i64;
        cur %= MOD;

        if open > 0 {
            cur += open * self.solve(i + 1, open - 1, new_k) as i64;
            cur %= MOD;
        }

        self.dp[i as usize][open as usize][k as usize] = cur as i32;
        return cur as i32;
    }
}

fn main() {
    let ints = read_ints();
    let n = ints[0];
    let k = ints[1];

    let mut a: Vec<i64> = vec![0];
    let mut input = read_ints();

    input.sort();

    for num in input {
        a.push(num);
    }

    let mut dp = DP::DP(n, &a, k);
    
    let mut ans: i64 = 0;

    for k in 0..k + 1 {
        ans += dp.solve(1, 0, k) as i64;
        ans %= MOD;
    }
    println!("{}", ans);
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_ints() -> Vec<i64> {
    read_str().split_whitespace().map(|x| x.parse().unwrap()).collect()
}
