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

fn is_kalindrome_rm_x(a: &Vec<u32>, mut i: usize, mut j: usize, x: u32) -> bool {
  while i < j {
    if a[i] == x {
      i = i + 1;
    } else if a[j] == x {
      j = j - 1;
    } else if a[i] == a[j] {
      i = i + 1;
      j = j - 1;
    } else {
      return false;
    }
  }

  return true;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();

    let mut i = 0;
    let mut j = n - 1;
    let mut is_kalindrome = true;
    while i < j {
      if a[i] == a[j] {
        i += 1;
        j -= 1;
      } else {
        if !is_kalindrome_rm_x(&a, i, j, a[i]) && !is_kalindrome_rm_x(&a, i, j, a[j]) {
          is_kalindrome = false;
        }
        break;
      }
    }

    if is_kalindrome {
      println!("YES");
    } else {
      println!("NO");
    }
  }

}
