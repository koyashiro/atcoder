use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        y: usize,
    }

    for ten_t in 0..n + 1 {
        for five_t in 0..n-ten_t + 1 {
            let one_t = n - ten_t - five_t;

            if 10000 * ten_t + 5000 * five_t+ 1000 * one_t == y {
                println!("{} {} {}", ten_t, five_t, one_t);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
