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

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  const MAX: usize = 50_000;
  let mut is_prime = vec![true; MAX];
  is_prime[0] = false;
  is_prime[1] = false;

  let max_check = (MAX as f64).sqrt().ceil() as usize;
  for n in 2..=max_check {
    if is_prime[n] {
      for i in (2*n..MAX).step_by(n) {
        is_prime[i] = false;
      }
    }
  }

  // for i in 0..1000 {
  //   if is_prime[i] {
  //     println!("{}", i);
  //   }
  // }

  let t = read_val_line::<u32>();
  for _ in 0..t {
    let d = read_val_line::<u32>();

    // Find two divisor
    let mut n = 1 + d;
    while !is_prime[n as usize] {
      n = n + 1;
    }

    let ans: u64 = n as u64;

    let mut n = n + d;
    while !is_prime[n as usize] {
      n = n + 1;
    }

    let ans = ans * n as u64;

    println!("{}", ans);
  }
}
