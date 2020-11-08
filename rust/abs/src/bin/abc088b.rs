use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.reverse();

    let mut alice = 0;
    let mut bob = 0;

    for (i, ai) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += ai;
        } else {
            bob += ai;
        }
    }

    let sub = alice - bob;
    println!("{}", sub);
}
