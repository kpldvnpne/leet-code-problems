fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let n = read_val_line::<usize>();
        let mut a = read_vector_line::<u32>();

        let mut n_ops = 0;
        for i in (0..(n-1)).rev() {
            if a[i+1] == 0 {
                n_ops = -1;
                break;
            }
            while a[i] >= a[i+1] {
                a[i] = a[i] / 2;
                n_ops += 1;
            }
        }

        println!("{n_ops}");
    }
}