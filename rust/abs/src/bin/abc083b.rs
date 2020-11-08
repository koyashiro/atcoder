use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut total = 0;

    for nx in 1..n + 1 {
        let digits: [usize; 5] = [
            nx / 1 % 10,
            nx / 10 % 10,
            nx / 100 % 10,
            nx / 1000 % 10,
            nx / 10000 % 10,
        ];
        let subtotal = digits.iter().sum();
        if a <= subtotal && subtotal <= b {
            total += nx;
        }
    }

    println!("{}", total);
}
