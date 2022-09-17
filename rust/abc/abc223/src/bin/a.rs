use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
    }

    let ans = if 100 <= x && x % 100 == 0 {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
