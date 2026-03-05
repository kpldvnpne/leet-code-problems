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
    let a = read_vector_line::<i64>();

    let yasser_sum = a.iter().sum::<i64>();
    
    if yasser_sum <= 0 {
      println!("NO");
    } else {
      let mut adel_sum_1 = 0;
      let mut adel_sum_2 = 0;
      let mut found = false;
      for (i, &a_i) in a.iter().enumerate() {
        // println!("i = {}, a_i = {}", i, a_i);
        if i != n - 1 {
          adel_sum_1 = adel_sum_1.max(0) + a_i;

          if adel_sum_1 >= yasser_sum {
            // println!("Found because of sum 1 at index {} with sum {} and yasser_sum {}", i, adel_sum_1, yasser_sum);
            found = true;
            break;
          }
        }

        if i != 0 {
          adel_sum_2 = adel_sum_2.max(0) + a_i;

          if adel_sum_2 >= yasser_sum {
            // println!("Found because of sum 2 at index {}", i);
            found = true;
            break;
          }
        }
      }

      if found {
        println!("NO");
      } else {
        println!("YES");
      }
    }
  }
}