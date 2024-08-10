use std::{cmp::max, io::{self}};

static MAXN: i64 = 100000 + 105;

#[derive(Clone, Copy)]
struct DATA {
    r: i64,
    h: i64,
    v: i64
}

struct ZIP {
    v: Vec<i64>
}

impl ZIP {
    fn zip() -> ZIP {
        ZIP {
            v: Vec::new()
        }
    }
    
    fn add(&mut self, val: i64) {
        self.v.push(val);
    }

    fn init(&mut self) {
        self.v.sort();
    }

    fn get(&mut self, val: i64) -> i64 {
        let mut pos: i64 = match self.v.binary_search(&val) {
            Ok(index) => index as i64,
            Err(index) => index as i64,
        };

        return pos + 1;
    }
}

struct FenwickTree {
    bit: Vec<i64>,
}

impl FenwickTree {
    fn fenwick_tree() -> FenwickTree {
        let bit = vec![0; MAXN as usize];

        FenwickTree {
            bit: bit
        }
    }

    fn update(&mut self, l: i64, r: i64, val: i64) {
        let mut id: i64 = l;
        while id < r {
            self.bit[id as usize] = max(self.bit[id as usize], val);
            id += id & -id;
        }
    }

    fn get(&mut self, mut u: i64) -> i64 {
        let mut ans: i64 = 0;
        while u > 0 {
            ans = max(ans, self.bit[u as usize]);
            u -= u & -u;
        }
        ans
    }
}
fn main() {
    let n = read_ints()[0];

    let mut a = vec![DATA { r: 0, h: 0, v: 0 }; (n + 1) as usize];

    let mut zip = ZIP::zip();

    for i in 1 .. n + 1 {
        let input = read_ints();
        a[i as usize].r = input[0];
        a[i as usize].h = input[1];
        a[i as usize].v = a[i as usize].h * a[i as usize].r * a[i as usize].r;
        zip.add(a[i as usize].v);
    }

    zip.init();

    let mut bit = FenwickTree::fenwick_tree();

    let mut dp = vec![0; (n + 1) as usize];

    let mut ans: i64 = 0;
    for i in 1 .. n + 1 {
        let mut id: i64 = zip.get(a[i as usize].v);
        dp[i as usize] = bit.get((id - 1) as i64) + a[i as usize].v;
        ans = max(ans, dp[i as usize]);
        bit.update(id, MAXN, dp[i as usize]);
    }

    let pi = std::f64::consts::PI;
    println!("{:.20}", (ans as f64 * pi) as f64);
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_ints() -> Vec<i64> {
    read_str().split_whitespace().map(|x| x.parse().unwrap()).collect()
}
