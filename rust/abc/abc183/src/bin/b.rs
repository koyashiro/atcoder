use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        sx: isize,
        sy: isize,
        gx: isize,
        gy: isize,
    }

    let dy: isize = sy + gy;
    let dx: isize = gx - sx;

    let x = dx as f64 * sy as f64 / dy as f64 + sx as f64;
    println!("{}", x);
}
