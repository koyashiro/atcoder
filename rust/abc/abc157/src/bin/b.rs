use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: [usize; 9],
        n: usize,
        b: [usize; n],
    }

    for bi in b {
        for ai in a.iter_mut() {
            if *ai == bi {
                *ai = 0;
            }
        }
    }

    let ans = if (a[0] == 0 && a[1] == 0 && a[2] == 0)
        || (a[3] == 0 && a[4] == 0 && a[5] == 0)
        || (a[6] == 0 && a[7] == 0 && a[8] == 0)
        || (a[0] == 0 && a[3] == 0 && a[6] == 0)
        || (a[1] == 0 && a[4] == 0 && a[7] == 0)
        || (a[2] == 0 && a[5] == 0 && a[8] == 0)
        || (a[0] == 0 && a[4] == 0 && a[8] == 0)
        || (a[2] == 0 && a[4] == 0 && a[6] == 0)
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
