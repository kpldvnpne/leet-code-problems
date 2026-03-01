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
    #[allow(non_snake_case)]
    let [_n, W] = read_vector_line::<u32>()[..2] else { panic!() };
    let w = read_vector_line::<u32>();

    use std::collections::BTreeMap;
    use std::cmp::Reverse;
    let mut w_multiset = BTreeMap::new();
    for w_i in w {
      let entry = w_multiset.entry(Reverse(w_i)).or_insert(0);
      *entry += 1;
    }

    let mut line_rem = W;
    let mut height = 1;
    while !w_multiset.is_empty() {
      let fit_w = w_multiset.range(Reverse(line_rem)..).next();
      if let Some((&Reverse(fit_w), &count)) = fit_w {
        // println!("{} on line {}", fit_w, height);
        line_rem -= fit_w;
        if count == 1{
          w_multiset.remove(&Reverse(fit_w));
        } else {
          w_multiset.entry(Reverse(fit_w)).and_modify(|c| *c -= 1);
        }
      } else {
        line_rem = W;
        height += 1;
      }
    }

    println!("{}", height);
  }
}
