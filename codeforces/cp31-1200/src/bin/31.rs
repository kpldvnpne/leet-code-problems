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

fn get_max_1bit_pos(n: u32) -> u32 {
  for i in (0..32).rev() {
    if (n >> i) & 1 == 1 {
      return i;
    }
  }

  unreachable!();
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();
    let mut counts = vec![0; 32];
    for a_i in a {
      counts[get_max_1bit_pos(a_i) as usize] += 1;
    }

    let mut ans: u64 = 0;
    for i in 0..32 {
      let count = counts[i] as u64;
      if count > 0 {
        ans += count * (count - 1) / 2;
      }
    }

    println!("{}", ans);
  }
}