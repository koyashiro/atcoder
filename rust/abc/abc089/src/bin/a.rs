use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let x = n / 3;

    println!("{}", x);
}
