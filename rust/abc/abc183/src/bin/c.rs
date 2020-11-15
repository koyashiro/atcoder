use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut routes: Vec<Vec<usize>> = Vec::new();
    for permutation in (2..n + 1).permutations(n - 1) {
        let mut route = Vec::new();

        route.push(1);
        permutation.iter().for_each(|i| {
            route.push(*i);
        });
        route.push(1);

        routes.push(route);
    }

    let mut cnt = 0;

    for route in routes {
        let mut dt: usize = 0;
        for n in 0..route.len() - 1 {
            let from = route[n];
            let to = route[n + 1];
            let t = t[from - 1][to - 1];
            dt += t;
        }
        if dt == k {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
