use proconio::input;

// e
fn main() {
    input! {
        n: String,
    }

    let n: Vec<u8> = n.as_bytes().iter().map(|x| x - 48).collect();

    let mut t = 500;
    for i in 1..n.len() {
        let prev = n[i - 1];
        let current = n[i];
        let dt = if prev == current {
            301
        } else if Hand::from(prev) == Hand::from(current) {
            210
        } else {
            100
        };
        t += dt;
    }
    println!("{}", t);
}

#[derive(Eq, PartialEq)]
enum Hand {
    Left,
    Right,
}

impl From<u8> for Hand {
    fn from(n: u8) -> Self {
        match n {
            1..=5 => Self::Left,
            6..=9 | 0 => Self::Right,
            _ => unreachable!(),
        }
    }
}
