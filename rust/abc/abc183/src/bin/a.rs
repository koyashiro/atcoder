use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: isize
    }

    let result = if x >= 0 { x } else { 0 };
    println!("{}", result);
}
