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

fn powi(base: usize, power: usize) -> usize {
  let mut prod = 1;
  for _ in 0..power {
    prod = prod * base;
  }
  return prod;
}

fn get_index(s: &[u8]) -> usize {
  let mut index = 0;
  for c in s {
    index = index * 26 + (*c as usize - 'a' as usize);
  }
  return index;
}

fn convert_to_str(mex: usize, len: usize) -> String {
  let mut s = vec![];
  let mut rem_mex = mex;
  for _ in 0..len {
    let rem = rem_mex % 26;
    let c = (rem as u8 + 'a' as u8) as char;
    s.push(c);
    rem_mex = rem_mex / 26;
  }

  return s.iter().rev().collect();
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let s = read_line();
    let s = s.trim().as_bytes();

    for len in 1..10 {
      let max = powi(26, len);
      let mut presence = vec![false; max];
      for i in 0..(n+1-len) {
        let index = get_index(&s[i..i+len]);
        presence[index] = true;
      }

      let mut found = false;
      for mex in 0..max {
        if !presence[mex] {
          println!("{}", convert_to_str(mex, len));
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }
  }
}
