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
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
      let a_i = read_vector_line::<u32>();
      a.push(a_i);
    }

    let mut used_k = 0;
    for i in 0..(n+1)/2 {
      let mir_i = n - 1 - i;
      let max_j = if i == n / 2 { n / 2 } else { n };
      for j in 0..max_j {
        let mir_j = n - 1 - j;
        if a[i][j] != a[mir_i][mir_j] {
          used_k += 1;
        }
      }
    }

    if used_k > k {
      println!("NO");
    } else {
      let rem_k = k - used_k;
      if n % 2 == 0 && rem_k % 2 == 1 {
        println!("NO");
      } else {
        println!("YES");
      }
    }
  }
}
