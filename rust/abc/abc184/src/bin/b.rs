use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        s: Chars,
    }

    let mut result = x;
    for c in s {
        if c == 'o' {
            result += 1;
        } else {
            if result > 0 {
                result -= 1;
            }
        }
    }
    println!("{}", result);
}
