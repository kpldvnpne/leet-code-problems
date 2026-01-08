pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let n = 4;
    let conflicting_pairs = vec![
      vec![2, 3],
      vec![1, 4]
    ];
    let actual = Solution::max_subarrays(n, conflicting_pairs);
    let expected = 9;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test2() {
    let n = 5;
    let conflicting_pairs = vec![
      vec![1, 2],
      vec![2, 5],
      vec![3, 5],
    ];
    let actual = Solution::max_subarrays(n, conflicting_pairs);
    let expected = 12;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test3() {
    let n = 10;
    let conflicting_pairs = vec![
      vec![10,5],
      vec![3,8]
    ];
    let actual = Solution::max_subarrays(n, conflicting_pairs);
    let expected = 50;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test4() {
    let n = 25;
    let conflicting_pairs = vec![
      vec![25, 18],
      vec![2, 13],
      vec![15, 12],
      vec![12, 15],
    ];
    let actual = Solution::max_subarrays(n, conflicting_pairs);
    let expected = 189;
    assert_eq!(actual, expected);
  }
}

// end is inclusive
fn count_subarrays(start: i32, end: i32, done_until: i32) -> i64 {
  // println!("Counting from {start} to {end}");
  let n = (end - start + 1) as i64;
  let n_done = if done_until >= start {(done_until - start + 1) as i64} else {0};
  return n * (n + 1) / 2 - n_done * (n_done + 1) / 2;
}

fn get_n_subarrays(n: i32, conflicting_pairs: &Vec<Vec<i32>>, pair_to_skip: &Vec<i32>) -> i64 {
  let mut start = 1;
  let mut n_subarrays: i64 = 0;
  let mut done_until = 0;
  for pair in conflicting_pairs {
    if std::ptr::eq(pair, pair_to_skip) {
      // println!("Pair {} {} was skipped", pair[0], pair[1]);
      continue;
    }

    if start >= pair[0] + 1 {
      continue;
    }

    let end = pair[1] - 1;
    n_subarrays += count_subarrays(start, end, done_until);
    done_until = std::cmp::max(end, done_until);
    start = pair[0] + 1;
  }

  n_subarrays += count_subarrays(start, n, done_until);

  return n_subarrays;
}

impl Solution {
    pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
      let mut max = i64::MIN;
      for pair in &mut conflicting_pairs {
        pair.sort();
      }
      conflicting_pairs.sort_by(|a, b| a[1].cmp(&b[1])); // Sort by ending period
      for pair in &conflicting_pairs {
        let n_subarrays = get_n_subarrays(n, &conflicting_pairs, pair);
        // println!("When pair {}, {} was removed, the n_subarrays = {}", pair[0], pair[1], n_subarrays);
        max = std::cmp::max(n_subarrays, max);
      }
      return max;
    }
}
