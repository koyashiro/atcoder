use proconio::input;

// d
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut sum: Vec<(usize, usize, usize, usize)> =
        (0..n).map(|i| (i + 1, a[i], b[i], a[i] + b[i])).collect();
    sum.sort_by(|a, b| b.3.cmp(&a.3).then(b.1.cmp(&a.1)).then(a.0.cmp(&b.0)));
    for s in sum {
        print!("{} ", s.0);
    }
}
