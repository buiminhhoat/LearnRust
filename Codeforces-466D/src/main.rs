use std::{cmp::max, f32::consts::E, io::{self}, usize::MAX};

static MAXN: usize = 2005;
static MOD: i64 = 1000000007;

struct DP {
    n: i64,
    a: Vec<i64>,
    dp: Vec<Vec<i64>>,
    h: i64
}

impl DP {
    fn DP(n: i64, a: &[i64], h: i64) -> DP {
        let mut dp = vec![vec![-1; MAXN]; MAXN];
        DP {
            n: n,
            a: a.to_vec(),
            dp: dp,
            h: h
        }
    }

    fn solve(&mut self, i: i64, open: i64) -> i64 {
        if i == self.n + 1 {
            if open == 0 {
                return 1;
            }
            return 0;
        }
        if (self.dp[i as usize][open as usize] != -1) {
            return self.dp[i as usize][open as usize];
        }
        let mut cur: i64 = 0;
        if (self.a[i as usize] + open + 1 == self.h) {
            cur += self.solve(i + 1, open + 1);
            cur %= MOD;

            cur += self.solve(i + 1, open);
            cur %= MOD;

            cur += open * self.solve(i + 1, open);
            cur %= MOD;
        }
        
        if (self.a[i as usize] + open == self.h) {
            cur += self.solve(i + 1, open);
            cur %= MOD;

            if open > 0 {
                cur += open * self.solve(i + 1, open - 1);
                cur %= MOD;
            }
        }
        self.dp[i as usize][open as usize] = cur;
        return cur;
    }
}
fn main() {
    let ints = read_ints();
    let n = ints[0];
    let h = ints[1];

    let mut a: Vec<i64> = vec![0];
    let mut input = read_ints();
    for num in input {
        a.push(num);
    }

    let mut dp = DP::DP(n, &a, h);
    
    println!("{}", dp.solve(1, 0));
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_ints() -> Vec<i64> {
    read_str().split_whitespace().map(|x| x.parse().unwrap()).collect()
}
