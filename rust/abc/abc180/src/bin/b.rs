use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }

    let m = x.iter().map(|n| n.abs()).sum::<i64>();
    let e = (x.iter().map(|n| n.pow(2)).sum::<i64>() as f64).sqrt();
    let c = x.iter().map(|n| n.abs()).max().unwrap();

    println!("{}", m);
    println!("{}", e);
    println!("{}", c);
}
