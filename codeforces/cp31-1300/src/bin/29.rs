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
  let n = read_val_line::<usize>();

  let mut nbrs = vec![vec![]; n];
  for _ in 0..n-1 {
    let [u, v] = read_vector_line::<usize>()[..2] else { panic!() };
    let u = u - 1;
    let v = v - 1;

    nbrs[u].push(v);
    nbrs[v].push(u);
  }

  let mut visited = vec![false; n];
  let mut queue = vec![0];
  visited[0] = true;
  let mut n_part1 = 1;
  let mut n_part2 = 0;
  let mut cur_part = 2;
  while !queue.is_empty() {
    let mut new_queue = vec![];
    for u in queue {
      for &v in &nbrs[u] {
        if !visited[v] {
          visited[v] = true;
          new_queue.push(v);
        }
      }
    }

    // println!("{:?}", new_queue);

    if cur_part == 2 {
      cur_part = 1;
      n_part2 += new_queue.len();
    } else {
      cur_part = 2;
      n_part1 += new_queue.len();
    }

    queue = new_queue;
  }

  // println!("{n_part1} {n_part2}");

  let ans = n_part1 as u64 * n_part2 as u64 - (n - 1) as u64;

  println!("{}", ans);
}
