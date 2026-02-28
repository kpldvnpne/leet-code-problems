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


fn main() {
  let max_s_len = 2 * 100_000;
  let mut factorials = Vec::with_capacity(max_s_len + 1);
  factorials.push(1);
  let mut factorial: u64 = 1;
  for i in 1..=(max_s_len as u64) {
    factorial = (factorial * i) % MOD;
    factorials.push(factorial);
  }

  let t = read_val_line::<u32>();
  for _ in 0..t {
    let s = read_line();
    let s = s.trim().as_bytes();
    let mut gaps = vec![];

    let mut roll = 0;
    let mut ans1 = 0;
    for i in 1..=s.len() {
      if i == s.len() || s[i] != s[i-1] {
        if roll > 0 {
          ans1 += roll;
          gaps.push(roll);
        }
        roll = 0;
      } else {
        roll += 1;
      }
    }

    let mut ans2 = 1;
    while !gaps.is_empty() {
      let mut new_gaps = vec![];
      for &gap in &gaps {
        ans2 = (ans2 * (gap + 1)) % MOD;
        if gap > 1 {
          new_gaps.push(gap - 1);
        }
      }
      ans2 = (ans2 * factorials[gaps.len()]) % MOD;
      gaps = new_gaps;
    }

    println!("{} {}", ans1, ans2);
  }
}
