use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        a: u64,
        b: u64,
    }

    let mut m = x - a;
    while m >= b {
        m -= b;
    }

    println!("{}", m);
}
