use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut c = 0;

    for si in s {
        if si == '1' {
            c += 1;
        }
    }

    println!("{}", c);
}
