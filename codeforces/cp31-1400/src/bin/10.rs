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
    let [n, m] = read_vector_line::<u64>()[..2] else { panic!() };

    let mut rem_n = n;
    while rem_n % 10 == 0 {
      rem_n /= 10;
    }

    let mut cad_m = 1;
    while rem_n % 2 == 0 && cad_m * 5 <= m {
      rem_n /= 2;
      cad_m *= 5;
    }

    while rem_n % 5 == 0 && cad_m * 2 <= m {
      rem_n /= 5;
      cad_m *= 2;
    }

    let mut i = 10;
    while i > 1 {
      if cad_m * i <= m {
        cad_m *= i;
      } else {
        i -= 1;
      }
    }

    let ans = cad_m * n;
    println!("{}", ans);
  }
}
