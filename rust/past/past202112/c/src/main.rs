use proconio::input;

// c
fn main() {
    input! {
        n: usize,
        pv: [(String, String); n],
    }
    let a = pv
        .iter()
        .enumerate()
        .find_map(|(i, (p, v))| {
            if p == "A" && v == "AC" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    let b = pv
        .iter()
        .enumerate()
        .find_map(|(i, (p, v))| {
            if p == "B" && v == "AC" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    let c = pv
        .iter()
        .enumerate()
        .find_map(|(i, (p, v))| {
            if p == "C" && v == "AC" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    let d = pv
        .iter()
        .enumerate()
        .find_map(|(i, (p, v))| {
            if p == "D" && v == "AC" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    let e = pv
        .iter()
        .enumerate()
        .find_map(|(i, (p, v))| {
            if p == "E" && v == "AC" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    let f = pv
        .iter()
        .enumerate()
        .find_map(|(i, (p, v))| {
            if p == "F" && v == "AC" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    println!("{}\n{}\n{}\n{}\n{}\n{}", a, b, c, d, e, f);
}
