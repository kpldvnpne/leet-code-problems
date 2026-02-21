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
    let [_n, m] = read_vector_line()[..2] else { panic!() };
    let a = read_vector_line::<u32>();
    let mut mods = vec![0; m];
    for &a_i in &a {
      mods[a_i as usize % m] += 1;
    }

    let mut ans = 0;
    if mods[0] > 0 {
      ans += 1;
    }

    for i in 1..=(m/2) {
      let left = mods[i];
      let right = mods[m-i];

      let min = left.min(right);
      let max = left.max(right);

      if min == 0 {
        ans += max;
      } else if min == max || min + 1 == max {
        ans += 1;
      } else {
        ans += 1 + max - (min + 1);
      }
    }

    println!("{}", ans);
  }
}