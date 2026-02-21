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
    let a = read_vector_line::<u32>();

    let mut equiv_classes = vec![0; 2 * n + 1];
    for (i, &a_i) in a.iter().enumerate() {
      let equiv_i = a_i as usize + (n - i - 1);
      equiv_classes[equiv_i] += 1;
    }

    let mut ans: u64 = 0;
    for equiv_val in equiv_classes {
      if equiv_val > 1 {
        ans += equiv_val * (equiv_val - 1) / 2;
      }
    }

    println!("{}", ans);
  }
}
