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

struct Location {
  x: i64,
  y: i64,
}

impl Location {
  fn distance_from(&self, other: &Location) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, k, a, b] = read_vector_line::<usize>()[..4] else { panic!() };
    let a = a - 1;
    let b = b - 1;
    let mut locations = vec![];
    for _ in 0..n {
      let [x, y] = read_vector_line::<i64>()[..2] else { panic!() };
      locations.push(Location {x, y});
    }

    let a_dist_from_major = if a < k { 0 } else {
      locations[..k].iter().map(|it| it.distance_from(&locations[a])).min().unwrap_or(1e12 as i64)
    };

    let b_dist_from_major = if b < k { 0 } else {
      locations[..k].iter().map(|it| it.distance_from(&locations[b])).min().unwrap_or(1e12 as i64)
    };

    let direct_distance = locations[a].distance_from(&locations[b]);


    let ans = (a_dist_from_major + b_dist_from_major).min(direct_distance);

    println!("{}", ans);
  }
}
