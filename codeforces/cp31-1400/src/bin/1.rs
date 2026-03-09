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
    let [_n, m] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();

    let mut trailing_zeros = vec![];
    let mut digits = 0;

    for a_i in a {
      let mut a_i = a_i;
      let mut n_0s = 0;
      while a_i > 0 {
        let rem = a_i % 10;
        if rem != 0 {
          break;
        }
        n_0s += 1;
        a_i = a_i / 10;
      }

      let mut n_rest = 0;
      while a_i > 0 {
        n_rest += 1;
        a_i = a_i / 10;
      }

      trailing_zeros.push(n_0s);
      digits += n_rest;
    }

    trailing_zeros.sort_by(|a, b| b.cmp(&a));
    digits += trailing_zeros.iter().skip(1).step_by(2).sum::<usize>();

    if digits > m {
      println!("Sasha");
    } else {
      println!("Anna");
    }
  }
}
