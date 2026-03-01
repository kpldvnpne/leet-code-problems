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

    const MAX_BITS: usize = 30;
    let mut bit_count = vec![0; MAX_BITS];
    for &a_i in &a {
      for i in 0..MAX_BITS {
        if (a_i >> i) & 1 == 1 {
          bit_count[i] += 1;
        }
      }
    }

    for k in 1..=n {
      let mut divides_all = true;
      for i in 0..MAX_BITS {
        if bit_count[i] % k != 0 {
          divides_all = false;
          break;
        }
      }

      if divides_all {
        print!("{} ", k);
      }
    }

    println!();
  }
}
