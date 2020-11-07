use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    let result = if a * b % 2 == 0 { "Even" } else { "Odd" };

    println!("{}", result);
}
