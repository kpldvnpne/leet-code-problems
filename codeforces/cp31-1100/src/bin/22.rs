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

fn bitpow(mut base: u64, mut pow: u64) -> u64 {
    let mut res = 1;
    base %= MOD; // Ensure base is within MOD initially

    while pow > 0 {
        // If the current bit is 1, multiply the current base power into the result
        if pow % 2 == 1 {
            res = (res * base) % MOD;
        }
        // Square the base for the next bit (base^1 -> base^2 -> base^4...)
        base = (base * base) % MOD;
        // Shift right to check the next bit
        pow >>= 1;
    }

    res
}

fn main() {
  let t = read_val_line::<u32>();
  let inv_6 = bitpow(6, MOD-2);

  for _ in 0..t {
    let n = read_val_line::<u64>();

    let mut sqs = (n * (n + 1)) % MOD;
    sqs = (sqs * (4*n - 1)) % MOD;
    sqs = (sqs * inv_6) % MOD;

    // println!("{}", sqs);

    let ans = (2022 * sqs) % MOD;

    println!("{}", ans);
  }
}
