use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let sab = a * b;
    let scd = c * d;
    let ans = if sab > scd { sab } else { scd };

    println!("{}", ans);
}
