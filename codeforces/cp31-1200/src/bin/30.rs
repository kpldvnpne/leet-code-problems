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
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();

    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &a_i) in a.iter().enumerate() {
      let e = map.entry(a_i).or_insert(vec![]);
      e.push(i+1);
    }

    if map.len() == 1 {
      println!("NO");
      continue;
    }

    let mut prev: Vec<usize> = vec![];
    let mut connecting = 0;
    let mut ans = vec![];
    for v in map.values() {
      let mut v_i = 0;
      while !prev.is_empty() && v_i < v.len() {
        let top = prev.pop().unwrap();
        if connecting != 0 {
          ans.push((top, connecting));
        }
        ans.push((top, v[v_i]));
        connecting = v[v_i];
        v_i += 1;
      }

      if v_i < v.len() && !ans.is_empty(){
        let last_top = ans.last().unwrap().0;
        connecting = last_top;
      }

      while v_i < v.len() {
        prev.push(v[v_i]);
        v_i += 1;
      }
    }

    while !prev.is_empty() {
      let top = prev.pop().unwrap();
      ans.push((top, connecting));
    }

    println!("YES");
    for (a, b) in ans {
      println!("{} {}", a, b);
    }
  }
}