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
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<u64>();
    let max_a = *a.iter().max().unwrap();

    let mut c = 2;
    while c <= max_a {
      let mut is_ans = true;
      use std::collections::HashSet;
      let mut set = HashSet::new();
      for item in &a {
        set.insert(item % c);
        if set.len() > 2 {
          break;
        }
      }

      if set.len() != 2 {
        is_ans = false;
      }
      
      if is_ans {
        break;
      }

      c = c << 1;
    }

    println!("{}", c);
  }
}