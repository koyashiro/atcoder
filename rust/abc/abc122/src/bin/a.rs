use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        b: String,
    }

    let ans = match b.as_str() {
        "A" => "T",
        "T" => "A",
        "C" => "G",
        "G" => "C",
        _ => unreachable!(),
    };

    println!("{}", ans);
}
