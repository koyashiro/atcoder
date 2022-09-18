use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let permutations = (1..n + 1).permutations(n).collect::<Vec<Vec<usize>>>();
    let a = permutations.iter().position(|perm| perm == &p).unwrap();
    let b = permutations.iter().position(|perm| perm == &q).unwrap();
    let ans = (a as isize - b as isize).abs();

    println!("{}", ans);
}
