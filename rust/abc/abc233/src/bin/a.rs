use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
    }

    let mut cnt = 0;
    while x + 10 * cnt < y {
        cnt += 1;
    }

    println!("{}", cnt);
}
