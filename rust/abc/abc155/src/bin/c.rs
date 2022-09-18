use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();

    for si in &s {
        let n = *map.get(&si).unwrap_or(&0);
        map.insert(si, n + 1);
    }

    let max = map.values().max().unwrap();
    let mut max_s = map
        .iter()
        .filter(|(_, v)| *v == max)
        .map(|(k, _)| k.as_str())
        .collect::<Vec<&str>>();
    max_s.sort();

    for v in max_s {
        println!("{}", v);
    }
}
