use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = if s < t { "Yes" } else { "No" };

    println!("{}", ans);
}
