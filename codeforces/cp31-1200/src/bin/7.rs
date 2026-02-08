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
    let [n, m] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut c = Vec::with_capacity(n);
    for _ in 0..n {
      let c_i = read_vector_line::<u32>();
      c.push(c_i);
    }

    let mut total_winnings: u64 = 0;
    for j in 0..m {
      let mut v = (0..n).map(|i| c[i][j]).collect::<Vec<_>>();
      v.sort();
      let mut i = 0;
      let mut j = n - 1;
      let mut cur_winnings: u64 = 0;
      while i < j {
        cur_winnings += (v[j] - v[i]) as u64 * (j - i) as u64;
        i = i + 1;
        j = j - 1;
      }

      total_winnings += cur_winnings;
    }

    println!("{}", total_winnings);
  }
}