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

enum Condition {
  Same,
  Invert,
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let _n = read_val_line::<usize>();
    let a = read_line();
    let a = a.trim().bytes();
    let b = read_line();
    let b = b.trim().bytes();

    let mut balance = 0;
    let mut cur_condition = Condition::Same;

    let mut possible = true;
    for (a_i, b_i) in a.zip(b) {
      if balance == 0 {
        cur_condition = if a_i == b_i { Condition::Same } else { Condition::Invert };
      } else {
        if matches!(cur_condition, Condition::Same) {
          if a_i != b_i {
            possible = false;
          }
        } else {
          if a_i == b_i {
            possible = false;
          }
        }
      }

      if a_i == '0' as u8 {
        balance -= 1;
      } else {
        balance += 1;
      }
    }

    if matches!(cur_condition, Condition::Invert) && balance != 0 {
      possible = false;
    }

    if possible {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}