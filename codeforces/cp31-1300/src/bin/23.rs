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
    let n = read_val_line::<u32>();
    let mut rem_n = n;
    let mut a_b = vec![];
    let mut f = 2;
    while f * f <= rem_n {
      if rem_n % f == 0 {
        rem_n = rem_n / f;
        if rem_n == f {
          break;
        }
        a_b.push(f);
        if a_b.len() == 2 {
          break;
        }
      }
      f = f + 1;
    }

    if a_b.len() == 2 {
      println!("YES");
      println!("{} {} {}", a_b[0], a_b[1], rem_n);
    } else {
      println!("NO");
    }
  }
}