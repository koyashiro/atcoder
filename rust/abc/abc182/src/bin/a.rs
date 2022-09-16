use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
    }

    let max = 2 * a + 100;
    let diff = max - b;

    println!("{}", diff);
}
