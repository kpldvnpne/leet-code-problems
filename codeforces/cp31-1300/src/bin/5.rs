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
fn factorial(n: u32) -> u64 {
  let n = n as u64;

  let mut prod = 1u64;
  for i in 2..=n {
    prod = (prod * i) % MOD;
  }
  return prod;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let s = read_line();
    let s = s.trim().as_bytes();

    let n = s.len();
    let mut run = 1;
    let mut ans_1 = 0;
    let mut ans_2 = 1;
    for i in 1..=n {
      if i == n || s[i] != s[i-1] {
        ans_1 += run - 1;
        ans_2 = (ans_2 * factorial(run)) % MOD;
        run = 1;
      } else {
        run += 1;
      }
    }

    println!("{} {}", ans_1, ans_2);
  }
}