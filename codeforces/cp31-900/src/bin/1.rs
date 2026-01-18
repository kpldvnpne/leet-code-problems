fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

use std::collections::HashSet;
fn get_knight_pos(x: i32, y: i32, a: i32, b: i32) -> HashSet<(i32, i32)> {
  let mut set = HashSet::with_capacity(4);

  for (a, b) in [(a, b), (b, a)] {
    for i in [-1, 1] {
      for j in [-1, 1] {
        set.insert((x + i * a, y + j * b));
      }
    }
  }

  set
}

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let [a, b] = read_vector_line::<i32>()[0..2] else { panic!() };
    let [x_k, y_k] = read_vector_line::<i32>()[0..2] else { panic!() };
    let [x_q, y_q] = read_vector_line::<i32>()[0..2] else { panic!() };

    let k_knight_pos = get_knight_pos(x_k, y_k, a, b);
    let q_knight_pos = get_knight_pos(x_q, y_q, a, b);

    let ans = k_knight_pos.intersection(&q_knight_pos).collect::<Vec<_>>().len();

    println!("{ans}");
  }
}
