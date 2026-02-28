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
    let a = read_vector_line::<usize>();
    let mut n_candidates = vec![0; n];
    let mut cur = 0u64;
    let mut ans = 0;
    for (i, &a_i) in a.iter().enumerate() {
      if a_i < i + 1 {
        cur += 1;

        if a_i >= 2 {
          ans += n_candidates[a_i - 1 - 1];
        }
      }
      n_candidates[i] = cur;
    }

    println!("{}", ans);
  }
}
