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
    let [n, _m] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut a = read_vector_line::<u32>();
    let mut b = read_vector_line::<u32>();

    let mut new_a = Vec::with_capacity(n);
    new_a.push(1);
    new_a.append(&mut a);
    let mut a = new_a;

    a.sort();
    b.sort();
    let a = a;
    let b = b;

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < n && j < n {
      if a[i] < b[j] {
        k += 1;
        i += 1;
        j += 1;
      } else {
        j = j + 1;
      }
    }

    println!("{}", n-k);
  }
}
