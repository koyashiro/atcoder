use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [u64; n],
    }

    p.sort();
    let ans = p.iter().take(k).sum::<u64>();

    println!("{}", ans);
}
