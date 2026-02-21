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

fn pow(b: u32, e: u32) -> u64 {
  let b = b as u64;
  let e = e as u64;
  let mut prod = 1;
  for _ in 0..e {
    prod = (prod * b) % MOD;
  }
  return prod;
}

#[allow(non_snake_case)]
fn nCr(n: u32, r: u32) -> u64 {
  if r > n { return 0 };
  if r == n || r == 0 { return 1 };
  let n = n as u64;
  let r = r as u64;
  let r = if r < (n - r) { r } else { n - r };
  let mut result = 1;
  for r_i in 0..r {
    result = result * (n - r_i) / (r_i + 1);
  }

  return result % MOD;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, k] = read_vector_line::<u32>()[..2] else { panic!() };
    let mut ans: u64 = 0;
    for k_i in 0..=k {
      let k_i_ans = (pow(n-1, k_i) * nCr(k, k_i)) % MOD;
      ans = (ans + k_i_ans) % MOD;
    }

    println!("{}", ans);
  }
}