use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut routes: Vec<Vec<usize>> = vec![];
    for i in (2..n + 1).permutations(n - 1) {
        let mut route = vec![1];
        for n in i {
            route.push(n);
        }
        route.push(1);
        routes.push(route);
    }

    let mut cnt = 0;

    for route in routes {
        let mut dt: usize = 0;
        for n in 0..route.len() - 1 {
            let from: usize = route[n];
            let to: usize = route[n + 1];
            let t: usize = t[(from - 1) as usize][(to - 1) as usize];
            dt += t;
        }
        if dt == k {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
