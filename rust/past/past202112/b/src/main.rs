use proconio::input;

// b
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let changes: Vec<usize> = ab.iter().map(|(a, b)| b - a).collect();

    let mut count = 0;
    for c in changes {
        let (d1, d2) = (c % 10, c / 10 % 10);
        if d1 >= 5 {
            count += 1;
        }
        if d2 >= 5 {
            count += 1;
        }
    }

    println!("{}", count);
}
