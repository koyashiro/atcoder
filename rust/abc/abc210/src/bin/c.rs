use std::{cmp::max, collections::HashMap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }

    let mut map = HashMap::new();
    for ci in c.as_slice()[0..k].iter() {
        let n = *map.get(ci).unwrap_or(&0);
        map.insert(*ci, n + 1);
    }
    let mut ans = map.len();

    for i in 1..=n - k {
        {
            let ci = c[i - 1];
            if let Some(&n) = map.get(&ci) {
                if n == 1 {
                    map.remove(&ci);
                } else {
                    map.insert(ci, n - 1);
                }
            }
        }

        {
            let ci = c[i + k - 1];
            let n = *map.get(&ci).unwrap_or(&0);
            map.insert(ci, n + 1);
        }

        ans = max(ans, map.len());
    }

    println!("{}", ans);
}
