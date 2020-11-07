use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut cnt = 0;

    while a.iter().all(|&c| c % 2 == 0) {
        for ai in &mut a {
            *ai /= 2;
        }
        cnt += 1;
    }

    println!("{}", cnt);
}
