use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
        s: String,
    }

    let mut chars = s.chars().collect::<Vec<char>>();
    chars[l - 1..=r - 1].reverse();
    let ans = chars.iter().collect::<String>();

    println!("{}", ans);
}
