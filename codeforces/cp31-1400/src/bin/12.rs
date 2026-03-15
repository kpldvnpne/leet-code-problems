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

fn heapify(v: &mut Vec<i32>, index: usize) {
  let left = 2 * index + 1;
  let right = 2 * index + 2;
  let n = v.len();
  if left < n && right < n {
    if v[index] < v[left] || v[index] < v[right] {
      if v[left] > v[right] {
        let temp = v[left];
        v[left] = v[index];
        v[index] = temp;
        heapify(v, left);
      } else {
        let temp = v[right];
        v[right] = v[index];
        v[index] = temp;
        heapify(v, right);
      }
    }
  } else if left < n {
    if v[index] < v[left] {
      let temp = v[left];
      v[left] = v[index];
      v[index] = temp;
      heapify(v, left);
    }
  }
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, _m] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<usize>();

    // Calculate how many tasks each worker has
    use std::collections::HashMap;
    let mut worker_queue = HashMap::new();
    for a_i in a {
      let a_i = a_i - 1; // Convert from 1-indexed to 0-indexed
      let entry = worker_queue.entry(a_i).or_insert(0);
      *entry += 1;
    }

    // We don't need to remember which worker a task belongs to
    let mut queue = worker_queue.values().map(|it| *it).collect::<Vec<_>>();
    // We need to make longer queues shorter first so that worker get to work their own tasks as much as possible
    queue.sort_by(|a, b| { b.cmp(&a) }); // Reverse sort

    let mut time = 0;
    // n_0s need not be stored as it is determined by that loop iteration
    let mut n_1s = 0; // Behind from current time by 1
    // We don't need any more than 2 because once there are any n_2s that can't be filled, all tasks have dried up

    // Loopo runs for at most mlogm times, which solves the problem in time
    while !queue.is_empty() {
      // println!("{} {:?}", time, queue);
      time = time + 1;
      let mut n_0s = 0; // When time is increased, workers behind from current time by 0 is 0

      // Assign to their own queue
      let mut new_queue = Vec::with_capacity(queue.len());
      for i in 0..queue.len() {
        let new_val = queue[i] - 1;
        if new_val > 0 {
          new_queue.push(new_val);
        }
        n_0s += 1;
      }

      // The previous n_1s are now n_2s because they have 1 extra from current time increase
      for _ in 0..n_1s {
        if new_queue.is_empty() {
          break;
        }

        new_queue[0] -= 1;
        // If it is 0, remove it, but first swap it with the end
        if new_queue[0] == 0 {
          let last_idx = new_queue.len() - 1;
          new_queue[0] = new_queue[last_idx];
          new_queue[last_idx] = 0;
          new_queue.pop();
        }
        heapify(&mut new_queue, 0);

        n_0s += 1;
      }

      n_1s = n - n_0s;

      queue = new_queue;
    }

    println!("{}", time);
  }
}
