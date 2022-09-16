use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let c = (a - b) / 3_f64 + b;

    println!("{}", c);
}
