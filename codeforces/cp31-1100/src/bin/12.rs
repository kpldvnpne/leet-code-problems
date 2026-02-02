fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
#[allow(dead_code)]
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

#[allow(dead_code)]
fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

const MOD: u64 = 1_000_000_000 + 7;

fn main() {
    let t = read_val_line::<u32>();
    for _ in 0..t {
        let n = read_val_line::<usize>();
        let mut a = read_vector_line::<u32>();
        let mut b = read_vector_line::<u32>();

        a.sort();
        b.sort();

        let mut ans: u64 = 1;
        let mut j = 0;
        for i in 0..n {
            while j < n && a[i] > b[j] {
                j = j + 1;
            }
            let options = (j as i32 - i as i32).max(0);
            ans = ans as u64 * options as u64 % MOD;
        }

        println!("{}", ans);
    }
}

// 2 4 5 6 8 9
// 1 1 3 4 5 6

// 2, 3, 4, 5, 6, 6
// 2, 1, 1, 1, 1, 0
// 2 * 2 * 4 * 