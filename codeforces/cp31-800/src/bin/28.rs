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
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let mut a = read_vector_line::<i32>();
    for i in 1..n {
      if a[i] > a[0] {
        let temp = a[0];
        a[0] = a[i];
        a[i] = temp;
      } else if a[i] < a[1] {
        let temp = a[1];
        a[1] = a[i];
        a[i] = temp;
      }
    }

    if a[0] == a[1] {
      println!("NO");
    } else {
      println!("YES");
      for item in &a {
        print!("{item} ");
      }
      println!("");
    }
  }
}
