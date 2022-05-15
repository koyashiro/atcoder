use proconio::input;

// a
fn main() {
    input! {
        hl: usize,
        wl: usize,
        h: usize,
        w: usize,
    }
    if hl <= h && wl >= w {
        println!("Yes")
    } else {
        println!("No")
    }
}
