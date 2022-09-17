use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }

    let mut products = Vec::new();
    for i in 0..=9 {
        for j in 0..=9 {
            products.push(i * j);
        }
    }

    let ans = if products.contains(&n) { "Yes" } else { "No" };

    println!("{}", ans);
}
