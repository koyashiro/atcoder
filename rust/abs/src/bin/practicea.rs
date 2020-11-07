use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        s: String
    }
    let sum = a + b + c;
    println!("{} {}", sum, s);
}
