use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let s = s
        .into_iter()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect::<Vec<String>>();

    let mut ans = 0;
    let mut map = HashMap::new();

    for si in &s {
        match map.get(si).map(|n| *n) {
            Some(n) => {
                map.insert(si, n + 1);
                ans += n;
            }
            None => {
                map.insert(si, 1u64);
            }
        }
    }

    println!("{}", ans);
}
