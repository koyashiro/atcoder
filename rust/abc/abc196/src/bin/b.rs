use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
    }

    let ans = x.split(".").next().unwrap();

    println!("{}", ans);
}
