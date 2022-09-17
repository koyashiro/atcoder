use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let min = 1 * a;
    let max = 6 * a;
    let ans = if min <= b && b <= max { "Yes" } else { "No" };

    println!("{}", ans);
}
