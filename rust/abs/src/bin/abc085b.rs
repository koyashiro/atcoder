use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }

    d.sort();
    d.dedup();
    let cnt = d.len();

    println!("{}", cnt);
}
