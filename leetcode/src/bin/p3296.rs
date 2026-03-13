pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let mountain_height = 4;
    let worker_times = vec![2, 1, 1];

    let expected = 3;
    let actual = Solution::min_number_of_seconds(mountain_height, worker_times);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let mountain_height = 10;
    let worker_times = vec![3, 2, 2, 4];

    let expected = 12;
    let actual = Solution::min_number_of_seconds(mountain_height, worker_times);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let mountain_height = 5;
    let worker_times = vec![1];

    let expected = 15;
    let actual = Solution::min_number_of_seconds(mountain_height, worker_times);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
      use std::collections::BinaryHeap;
      use std::cmp::Reverse;

      let mut queue = BinaryHeap::new();

      for time in worker_times {
        queue.push(Reverse((time as i64, time as i64, 1 as i64)));
      }

      let mut time = 0;
      for _ in 0..mountain_height {
        let Some(Reverse((new_time, iter_time, mut iteration))) = queue.pop() else { panic!() };
        time = time.max(new_time);
        iteration += 1;
        queue.push(Reverse((new_time + iter_time * iteration, iter_time, iteration)));
      }

      return time;
    }
}
