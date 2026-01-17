fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn abs_diff(a: usize, b: usize) -> usize {
    use std::cmp::{max, min};
    max(a, b) - min(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let n = read_val_line::<usize>();
        let p = read_vector_line::<usize>();

        let mut max_len = None;
        for i in 0..n {
            let p_i = p[i];

            if i + 1 != p_i {
                let req_len = abs_diff(p_i, i + 1);
                let new_max_len = if let Some(max_len) = max_len {
                    gcd(max_len, req_len)
                } else {
                    req_len
                };
                max_len = Some(new_max_len);
            }
        }

        let ans = max_len.unwrap(); // It exists, because permutation is unsorted
        println!("{ans}");
    }
}