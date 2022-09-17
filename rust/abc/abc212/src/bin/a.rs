use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let ans = match (a, b) {
        (_, 0) => "Gold",
        (0, _) => "Silver",
        _ => "Alloy",
    };

    println!("{}", ans);
}
