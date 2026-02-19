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

    let mut ans = 0;
    for (i, a_i) in a.iter().enumerate() {
      let i = i + 1;
      let max_a_j = 2 * n / a_i;
      for a_j in 1..=max_a_j {
        if a_j * a_i <= i {
          continue;
        }
        let j = a_j * a_i - i;
        if j < i {
          continue;
        }
        if j > n {
          break;
        }
        if a[j-1] == a_j && i != j {
          ans += 1;
        }
      }
    }

    println!("{}", ans);
  }
}
