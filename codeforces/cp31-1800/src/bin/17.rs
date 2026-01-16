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
    let n_k_x = read_vector_line::<i32>();
    let [n, k, x] = n_k_x[0..3] else { panic!() };
    if x != 1 {
      println!("YES");
      println!("{n}");
      for _ in 0..n {
        print!("1 ");
      }
      println!("");
    } else {
      if k == 1 {
        println!("NO");
      } else {
        if k == 2 {
          if n % 2 == 0 {
            println!("YES");
            println!("{}", n/2);
            for _ in 0..(n/2) {
              print!("2 ")
            }
            println!("")
          } else {
            println!("NO");
          }
        } else {
          // k >= 3
          if n == 3 {
            println!("YES");
            println!("1");
            println!("3")
          } else if n % 2 == 0 {
            println!("YES");
            println!("{}", n / 2);
            for _ in 0..(n/2) {
              print!("2 ");
            }
            println!("");
          } else {
            let n_2s = (n - 3) / 2;
            println!("YES");
            println!("{}", n_2s + 1);
            for _ in 0..n_2s {
              print!("2 ")
            }
            println!("3");
          }
        }
      }
    }
  }
}
