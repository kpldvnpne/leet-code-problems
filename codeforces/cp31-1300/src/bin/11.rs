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
    let s = read_line();
    let s = s.trim().as_bytes();

    let mut n_whites = vec![0; n + 1];
    let mut n_blacks = vec![0; n + 1];

    let mut ans = 0;
    for i in (1..=n).rev() {
      let i_s = i - 1;
      if s[i_s] == 'W' as u8 {
        n_whites[i] += 1;
      } else if s[i_s] == 'B' as u8 {
        n_blacks[i] += 1;
      }

      if n_whites[i] == n_blacks[i] {
        ans += 1;
      }

      if i >= 2 {
        let i_a = i - 2;
        let a_i = a[i_a];
        n_whites[a_i] += n_whites[i];
        n_blacks[a_i] += n_blacks[i];
      }
    }

    // println!("{:?}\n{:?}", n_whites, n_blacks);

    println!("{}", ans);
  }
}
