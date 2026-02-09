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
    let [n, m] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut a = read_vector_line::<u32>();
    a.sort();
    let mut gaps = vec![];
    gaps.push(a[0] - 1 + n as u32 - a[m-1]);
    for i in 1..m {
      gaps.push(a[i] - a[i-1] - 1);
    }

    gaps.sort_by(|a, b| b.cmp(a));
    let mut infected = 0;
    let mut uninfected = 0;
    for gap in gaps {
      if gap <= infected {
        break;
      }

      let avail_gap = gap - infected;
      if avail_gap == 1 || avail_gap == 2 {
        uninfected += 1;
        infected += 2;
      } else {
        uninfected += avail_gap - 1;
        infected += 4;
      }
    }

    let ans = n as u32 - uninfected;
    println!("{}", ans);
  }
}

// 9 10 1 2; 4 5; 7
// 6 1  3 4
// 13 14 15 16 17 18 19 20 1 2 ; 4 5 6; 8 9 10 11;