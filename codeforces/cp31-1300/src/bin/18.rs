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
fn own_slower() {
  let n: i32 = read_val_line::<i32>();
  let mut ans = 0;
  for i in -n..=n {
    for j in -n..=n {
      let i = i.abs();
      let j = j.abs();

      if i % 2 == j % 2 {
        let max = i.max(j);
        let min_steps = 2 * max;
        if min_steps <= n && (n - min_steps) % 4 == 0 {
          ans += 1;
        }
      } else {
        let max = i.max(j);
        let min_steps_1 = 2 * max - 1;
        let min_steps_2 = 2 * max + 1;
        if (min_steps_1 <= n && (n - min_steps_1) % 4 == 0) || (min_steps_2 <= n && (n - min_steps_2) % 4 == 0) {
          ans += 1;
        }
      }
    }
  }

  println!("{}", ans);
}

fn main() {
  let n: i32 = read_val_line::<i32>();
  let ans = if n % 2 == 0 {
    (n / 2 + 1) * (n / 2 + 1)
  } else {
    2 * (n / 2 + 1) * (n / 2 + 2)
  };
  println!("{}", ans);
}