use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        x: usize,
        a: [usize; n],
    }

    let mut ans = x;
    for i in 0..n {
        let mut j = 1;
        while j <= d {
            ans += 1;
            j += a[i];
        }
    }
    println!("{}", ans);
}
