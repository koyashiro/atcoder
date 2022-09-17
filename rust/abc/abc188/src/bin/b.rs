use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut ip = 0;
    for i in 0..n {
        ip += a[i] * b[i];
    }

    let ans = if ip == 0 { "Yes" } else { "No" };

    println!("{}", ans);
}
