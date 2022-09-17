use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    for i in 1..=n {
        if i.is_even() {
            continue;
        }

        let mut cnt = 0;
        for j in 1..=i {
            if i % j == 0 {
                cnt += 1;
            }
        }
        if cnt == 8 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
