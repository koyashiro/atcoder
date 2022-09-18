use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut set = HashSet::new();

    for ai in a {
        if let Some(_) = set.get(&ai) {
            set.remove(&ai);
        } else {
            set.insert(ai);
        }
    }

    let ans = set.len();

    println!("{}", ans);
}
