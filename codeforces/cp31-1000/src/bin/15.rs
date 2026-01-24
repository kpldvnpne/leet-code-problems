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
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let line = read_line();
    let line = line.trim().as_bytes();

    let mut n_whites = 0;
    for i in 0..k {
      if line[i] == 'W' as u8 {
        n_whites += 1;
      }
    }

    let mut min_n_whites = n_whites;
    for i in k..n {
      if line[i-k] == 'W' as u8 {
        n_whites -= 1;
      }
      if line[i] == 'W' as u8 {
        n_whites += 1;
      }

      min_n_whites = std::cmp::min(n_whites, min_n_whites);
    }

    println!("{}", min_n_whites);

  }
}
