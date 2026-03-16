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
  let [n, _m] = read_vector_line::<usize>()[..2] else { panic!() };
  let mut color: Vec<(Vec<usize>, Vec<usize>)> = vec![(vec![], vec![]); 100_000];
  for i in 0..n {
    let mat_i = read_vector_line::<usize>();
    for (j, &mat_ij) in mat_i.iter().enumerate() {
      let c = mat_ij - 1;
      color[c].0.push(i);
      color[c].1.push(j);
    }
  }

  let mut ans: u64 = 0;
  for c in 0..100_000 {
    if color[c].0.len() <= 1 {
      continue;
    }

    color[c].0.sort();
    color[c].1.sort();

    fn get_dist(v: &Vec<usize>) -> u64 {
      let mut sum: u64 = 0;
      for i in 1..v.len() {
        sum += (v[i] - v[i-1]) as u64 * (v.len() - i) as u64 * i as u64;
      }
      return sum;
    }

    ans += get_dist(&color[c].0) + get_dist(&color[c].1);
  }

  println!("{}", ans);
}
