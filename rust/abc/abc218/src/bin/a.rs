use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }

    let ans = match s.as_bytes()[n - 1] {
        b'o' => "Yes",
        b'x' => "No",
        _ => unreachable!(),
    };

    println!("{}", ans);
}
