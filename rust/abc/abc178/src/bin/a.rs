use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
    }

    let ans = match x {
        0 => 1,
        1 => 0,
        _ => unreachable!(),
    };

    println!("{}", ans);
}
