use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
    }

    let mut cnt = 0u64;
    let mut m = 100u64;

    while x > m {
        cnt += 1;
        m += m / 100;
    }

    println!("{}", cnt);
}
