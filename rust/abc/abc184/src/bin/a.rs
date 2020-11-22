use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    let result = a * d - b * c;
    println!("{}", result);
}
