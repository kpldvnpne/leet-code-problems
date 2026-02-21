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

    // Convert a into next change
    let mut next_change = vec![-1; n];
    let mut last_i = 0;
    for i in 1..n {
      if a[i] != a[i-1] {
        while last_i < i {
          next_change[last_i] = i as i32;
          last_i += 1;
        }
      }
    }

    let q = read_val_line::<u32>();
    for _ in 0..q {
      let [l, r] = read_vector_line::<usize>()[..2] else { panic!() };
      let l = l - 1;
      let r = r - 1;

      let next = next_change[l];

      if next == -1 || next as usize > r {
        println!("-1 -1");
      } else {
        println!("{} {}", l + 1, next + 1);
      }
    }
  }
}