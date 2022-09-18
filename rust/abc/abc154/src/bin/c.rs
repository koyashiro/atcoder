use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let len = a.len();
    a.sort();
    a.dedup();
    let len_deduped = a.len();
    let ans = if len == len_deduped { "YES" } else { "NO" };

    println!("{}", ans);
}
