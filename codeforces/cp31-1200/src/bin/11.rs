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

// Took 23 mins
fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    #[allow(non_snake_case)]
    let T = read_line();
    #[allow(non_snake_case)]
    let T = T.trim().as_bytes();
    
    let mut min_k = vec![0; n];
    let mut min_cost = 0;
    for (i, c) in T.iter().enumerate() {
      if *c == b'0' {
        let mut j = i;
        while j < n && T[j] == b'0' {
          if min_k[j] == 0 {
            min_k[j] = i + 1;
          }
          j += i + 1;
        }
        min_cost += min_k[i];
      }
    }
    println!("{}", min_cost);
  }
}
