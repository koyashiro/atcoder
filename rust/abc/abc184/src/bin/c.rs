use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r1: isize,
        c1: isize,
        r2: isize,
        c2: isize,
    }

    let offsets = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (0, -2),
        (0, 2),
        (-2, 0),
        (2, 0),
        (0, -3),
        (0, 3),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (-2, -1),
        (-2, 1),
        (2, -1),
        (2, 1),
        (-3, 0),
        (3, 0),
    ];

    if r1 == r2 && c1 == c2 {
        println!("0");
        return;
    }

    if r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 {
        println!("1");
        return;
    }

    for offset in offsets.iter() {
        if r1 + offset.0 == r2 && c1 + offset.1 == c2 {
            println!("1");
            return;
        }
    }

    for offset in offsets.iter() {
        if r1 + offset.0 + c1 + offset.1 == r2 + c2 || r1 + offset.0 - (c1 + offset.1) == r2 - c2 {
            println!("2");
            return;
        }
    }

    for offset0 in offsets.iter() {
        for offset1 in offsets.iter() {
            if r1 + offset0.0 + offset1.0 == r2 && c1 + offset0.1 + offset1.1 == c2 {
                println!("2");
                return;
            }
        }
    }

    if (r1.abs() % 2 == c1.abs() % 2) == (r2.abs() % 2 == c2.abs() % 2) {
        println!("2");
        return;
    }

    println!("3");
}
