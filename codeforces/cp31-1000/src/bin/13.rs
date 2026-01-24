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
        let [n, x] = read_vector_line::<u32>()[..2] else { panic!() };
        let a= read_vector_line::<u32>();

        let mut min = a[0];
        let mut max = a[0];
        let mut n_changes = 0;
        for i in 1..(n as usize) {
            min = std::cmp::min(a[i], min);
            max = std::cmp::max(a[i], max);

            if max - min > 2 * x {
                min = a[i];
                max = a[i];
                n_changes += 1;
            }
        }

        println!("{n_changes}");
    }
}