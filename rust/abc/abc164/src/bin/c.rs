use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }

    s.sort();
    s.dedup();
    let ans = s.len();

    println!("{}", ans);
}
