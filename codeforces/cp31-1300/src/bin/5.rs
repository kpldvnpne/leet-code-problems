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

const MOD: u64 = 998244353;


fn factorial(n: u64) -> u64 {
  let n = n as u64;
  let mut ans = 1;
  for i in 2..=n {
    ans = (ans * i) % MOD;
  }
  return ans as u64;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let s = read_line();
    let s = s.trim().as_bytes();
    let mut lens = vec![];

    let mut roll = 1;
    let mut k = 0;
    for i in 1..=s.len() {
      if i == s.len() || s[i] != s[i-1] {
        if roll > 0 {
          k += 1;
          lens.push(roll);
        }
        roll = 1;
      } else {
        roll += 1;
      }
    }

    let n = s.len();
    let ans1 = (n - k) as u64;
    let lens_prod = lens.iter().fold(1u64, |a, &b| { (a * b) % MOD });
    let ans2 = (lens_prod * factorial(ans1)) % MOD;

    println!("{} {}", ans1, ans2);
  }
}
