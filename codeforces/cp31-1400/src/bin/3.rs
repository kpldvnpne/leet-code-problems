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
    let mut levels = Vec::with_capacity(n);
    for _ in 0..n {
      let [li, ri] = read_vector_line::<i64>()[..2] else { panic!() };
      levels.push((li, ri));
    }

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
      let k = (left + right) / 2;

      let mut is_possible = true;
      let mut cur_pos = (0, 0);
      for &(li, ri) in &levels {
        let new_pos = (cur_pos.0 - k, cur_pos.1 + k);
        let new_pos = (new_pos.0.max(li), new_pos.1.min(ri));

        if new_pos.0 > new_pos.1 {
          is_possible = false;
          break;
        }
        cur_pos = new_pos;
      }

      if is_possible {
        right = k;
      } else {
        left = k + 1;
      }
    }

    let ans = left;
    println!("{}", ans);
  }
}
