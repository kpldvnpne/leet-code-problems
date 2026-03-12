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
    let mut tree = vec![vec![]; n];
    for i in 0..n-1 {
      let [u, v] = read_vector_line::<usize>()[..2] else { panic!() };
      let u = u - 1;
      let v = v - 1;
      tree[u].push((v, i+1));
      tree[v].push((u, i+1));
    }

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    queue.push_back((0, 0, 1));
    visited[0] = true;

    let mut ans = 1;
    while let Some((v, insert_index, iteration)) = queue.pop_front() {
      visited[v] = true;
      for &(child, child_insert_index) in &tree[v] {
        if visited[child] {
          continue;
        }

        let mut child_iteration = iteration;
        if child_insert_index < insert_index {
          child_iteration += 1;
          ans = ans.max(child_iteration);
        }
        queue.push_back((child, child_insert_index, child_iteration));
      }
    }

    println!("{}", ans);
  }
}
