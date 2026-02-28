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
    let mut set_bitpos_list = Vec::with_capacity(n);
    let mut count_at_bitpos = vec![0; 200_000 + 1];
    for _i in 0..n {
      let set_bitpos = read_vector_line::<usize>();
      let set_bitpos = set_bitpos.into_iter().skip(1).collect::<Vec<_>>();

      for &bitpos in &set_bitpos {
        count_at_bitpos[bitpos] += 1;
      }

      set_bitpos_list.push(set_bitpos);
    }

    let mut found_ans = false;
    for set_bitpos in set_bitpos_list {
      let mut possible = true;
      for bitpos in set_bitpos {
        if count_at_bitpos[bitpos] == 1 {
          possible = false;
          break;
        }
      }
      if possible {
        found_ans = true;
        break;
      }
    }

    if found_ans {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
