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

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let w = read_vector_line::<u32>();

    let mut alice_ate = 0;
    let mut bob_ate = 0;
    let mut a_i = 0;
    let mut b_i = n - 1;
    let mut max_candies = 0;
    while a_i <= b_i {
      alice_ate += w[a_i];
      a_i += 1;
      while a_i <= b_i && alice_ate < bob_ate {
        alice_ate += w[a_i];
        a_i += 1;
      }

      while a_i <= b_i && bob_ate < alice_ate {
        bob_ate += w[b_i];
        b_i -= 1;
      }

      if alice_ate == bob_ate {
        max_candies = max_candies.max(a_i + n-1-b_i);
      }
    }

    println!("{}", max_candies);
  }
}