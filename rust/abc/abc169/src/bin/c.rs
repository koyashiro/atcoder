use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: f64,
    }

    let ans = a * (b * 100f64).round() as usize / 100;

    println!("{}", ans);
}
