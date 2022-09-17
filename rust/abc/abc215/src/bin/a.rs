use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let ans = if s.as_str() == "Hello,World!" {
        "AC"
    } else {
        "WA"
    };

    println!("{}", ans);
}
