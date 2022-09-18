use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let x = xy.iter().map(|xyi| xyi.0).collect::<Vec<isize>>();
    let y = xy.iter().map(|xyi| xyi.1).collect::<Vec<isize>>();

    let ps = (0..n).permutations(n).collect::<Vec<Vec<usize>>>();

    let mut sum = 0f64;
    for p in &ps {
        for ni in 0..n - 1 {
            let i = p[ni];
            let j = p[ni + 1];
            sum += (((x[i] - x[j]).pow(2) + (y[i] - y[j]).pow(2)) as f64).sqrt();
        }
    }

    let ans = sum / ps.len() as f64;

    println!("{}", ans);
}
