use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut cnt = 0;

    for ax in 0..a + 1 {
        for bx in 0..b + 1 {
            for cx in 0..c + 1 {
                if 500 * ax + 100 * bx + 50 * cx == x {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
