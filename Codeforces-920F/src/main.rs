use std::{cmp::max, io::{self}};

struct SegmentTree {
    a: Vec<i64>,
    st: Vec<i64>,
    max: Vec<i64>,
    d: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    fn init(a: &[i64], d: &[i64]) -> SegmentTree {
        let n = a.len();
        let mut st = vec![0; 4 * n + 1];
        let mut max = vec![0; 4 * n + 1];
        SegmentTree {
            a: a.to_vec(), 
            st: st,
            max: max,
            d: d.to_vec(),
            n: n
        }
    }

    fn build(&mut self, id: usize, l: usize, r: usize) {
        if l == r {
            self.st[id] = self.a[l - 1];
            self.max[id] = self.a[l - 1];
            return;
        }
        let mid = mid(l, r);
        self.build(left(id), l, mid);
        self.build(right(id), mid + 1, r);
        self.st[id] = self.st[left(id)] + self.st[right(id)];
        self.max[id] = max(self.max[left(id)], self.max[right(id)]);
    }

    fn update(&mut self, id: usize, l: usize, r: usize, u: usize, v: usize) {
        if v < l || r < u {
            return;
        }
        if self.max[id] <= 2 {
            return;
        }
        if l == r {
            self.st[id] = self.d[self.a[l - 1] as usize];
            self.max[id] = self.d[self.a[l - 1] as usize];
            self.a[l - 1] = self.d[self.a[l - 1] as usize];
            return;
        }
        let mid = mid(l, r);
        self.update(left(id), l, mid, u, v);
        self.update(right(id), mid + 1, r, u, v);
        self.st[id] = self.st[left(id)] + self.st[right(id)];
        self.max[id] = max(self.max[left(id)], self.max[right(id)]);
    }

    fn get(&mut self, id: usize, l: usize, r: usize, u: usize, v: usize) -> i64 {
        if v < l || r < u {
            return 0
        }
        if u <= l && r <= v {
            return self.st[id]
        }
        let mid = mid(l, r);
        return self.get(left(id), l, mid, u, v) + self.get(right(id), mid + 1, r, u, v);
    }
}

fn main() {
    let ints = read_ints();
    let n = ints[0];
    let m = ints[1];
    let a = read_ints();
    let mut d = vec![0; 1000005];

    for i in 1..1000000+1 {
        let mut j = i;
        while j < 1000000+1 {
            d[j] += 1;
            j += i;
        }
    }

    let mut segment_tree = SegmentTree::init(&a, &d);

    segment_tree.build(1, 1, n as usize);

    for i in 1..m+1 {
        let query = read_ints();
        let t = query[0];
        let l = query[1];
        let r = query[2];

        if t == 1 {
            segment_tree.update(1 as usize, 1 as usize, n as usize, l as usize, r as usize);
        }
        else {
            println!("{}", segment_tree.get(1 as usize, 1 as usize, n as usize, l as usize, r as usize));
        }
    }
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_ints() -> Vec<i64> {
    read_str().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn left(id: usize) -> usize {
    id << 1
}

fn right(id: usize) -> usize {
    id << 1 | 1
}

fn mid(l: usize, r: usize) -> usize {
    (l + r) >> 1
}