use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = if s.as_bytes()[..] == t.as_bytes()[..t.len() - 1] {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
