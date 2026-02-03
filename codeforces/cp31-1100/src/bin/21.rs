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

fn gcd(a: u64, b: u64) -> u64 {
  if a == 0 {
    return b;
  }

  if b == 0 {
    return a;
  }

  if a % b == 0 {
    return b;
  } else {
    return gcd(b, a % b);
  }
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();
    let mut presence = vec![-1; 1000 + 1];
    for (i, a_i) in a.iter().enumerate() {
      presence[*a_i as usize] = i as i32;
    }

    let mut v = vec![];
    for i in 1..=1000 {
      if presence[i] != -1 {
        v.push((i as u64, presence[i]));
      }
    }

    v.sort_by(|a, b| b.1.cmp(&a.1));

    let mut max = -1;
    for i in 0..v.len() {
      let a = v[i];
      for j in i..v.len() {
        let b = v[j];
        if gcd(a.0, b.0) == 1 {
          max = max.max(a.1 + 1 + b.1 + 1);
        }
      }
    }

    println!("{}", max);
  }
}
