use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }

    let mut total = 0;
    let mut ans = -1;

    for i in 0..n {
        let (v, p) = vp[i];
        total += v * p;

        if total > x * 100 {
            ans = i as isize + 1;
            break;
        }
    }

    println!("{}", ans);
}
