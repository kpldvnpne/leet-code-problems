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
    let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();

    let mut bits_count = vec![0; 31];
    for &a_i in &a {
      for bit_pos in 0..31 {
        if (a_i >> bit_pos) & 1 == 1 {
          bits_count[bit_pos] += 1;
        }
      }
    }

    let mut k = k;
    let mut ans = 0;
    for i in (0..31).rev() {
      let req = n - bits_count[i];
      ans = ans << 1;
      if k >= req {
        ans = ans + 1;
        k = k - req;
      }
    }

    println!("{}", ans);
  }
}
