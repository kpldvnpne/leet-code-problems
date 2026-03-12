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
    let s = read_line();
    let s = s.trim().as_bytes();

    if n % 2 == 1 {
      println!("-1");
      continue;
    }

    let mut possible = true;
    let mut open = s[0];
    // let mut end = if s[0] == '(' as u8 { ')' as u8 } else { '(' as u8 };
    let mut colors = vec![0; n];
    let mut color = 1;

    loop {
      let mut has_more = false;
      let mut stack = vec![];
      for (i, &c) in s.iter().enumerate() {
        if colors[i] != 0 {
          continue;
        }
        if c == open {
          stack.push(i);
        } else {
          if !stack.is_empty() {
            let j = stack.pop().unwrap();
            colors[i] = color;
            colors[j] = color;
          } else {
            has_more = true;
            if color == 2 {
              break;
            }
          }
        }
      }

      has_more = has_more || !stack.is_empty();

      if !has_more {
        break;
      }

      if has_more && color == 2 {
        possible = false;
        break;
      }

      color = color + 1;
      open = if open == '(' as u8 { ')' as u8 } else { '(' as u8 };
      // end = if open == '(' as u8 { ')' as u8 } else { '(' as u8 };
    }

    if !possible {
      println!("-1");
    } else {
      println!("{}", color);
      for c in colors {
        print!("{} ", c);
      }
      println!();
    }
  }
}
