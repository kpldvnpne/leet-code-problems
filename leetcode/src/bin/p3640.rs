pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![0, -2, -1, -3, 0, 2, -1];
    let expected = -4;
    let actual = Solution::max_sum_trionic(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![1, 4, 2, 7];
    let expected = 14;
    let actual = Solution::max_sum_trionic(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = vec![-754,167,-172,202,735,-941,992];
    let expected = 988;
    let actual = Solution::max_sum_trionic(nums);

    assert_eq!(expected, actual);
  }
}

enum State {
  Increasing,
  Decreasing,
  Invalid,
}

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
      let a = nums;
      let n = a.len();

      let mut state = State::Invalid;
      let mut cur_sum: i64 = 0;
      let mut for_before: i64 = 0;
      let mut for_after: i64 = 0;
      let mut prevs: Vec<i64> = vec![];
      let mut max_sum: i64 = i64::MIN;
      let mut prev_first_item: i64 = 0;
      let mut cur_first_item: i64 = 0;
      for i in 1..n {
        if matches!(state, State::Invalid) && a[i-1] >= a[i] {
          continue; // Skip until increasing
        }

        if a[i-1] == a[i] {
          state = State::Invalid;
          prevs = vec![];
          cur_sum = 0;
          for_before = 0;
          for_after = 0;
          continue;
        } else if a[i-1] < a[i] {
          if matches!(state, State::Invalid) {
            state = State::Increasing;
            cur_sum = a[i] as i64;
            for_before = cur_sum;
            for_after = cur_sum;
            prevs = vec![];
            cur_first_item = a[i-1] as i64;
          } else if matches!(state, State::Increasing) {
            cur_sum += a[i] as i64;
            for_before = for_before.max(cur_sum);
            if cur_first_item + for_after + a[i] as i64 > a[i - 1] as i64 + a[i] as i64 {
              for_after = for_after + a[i] as i64
            } else {
              for_after = a[i] as i64;
              cur_first_item = a[i-1] as i64
            }
          } else if matches!(state, State::Decreasing) {
            state = State::Increasing;
            prevs.push(cur_sum);
            prev_first_item = cur_first_item;
            cur_first_item = a[i-1] as i64;

            cur_sum = a[i] as i64;
            for_before = a[i] as i64;
            for_after = a[i] as i64;
          }

          if prevs.len() >= 2 {
            let last_two = prevs[prevs.len()-2..prevs.len()].iter().sum::<i64>();
            max_sum = max_sum.max(prev_first_item + last_two + for_before);
          }
        } else { // Only option is a[i-1] > a[i]
          assert!(a[i-1] > a[i]);
          assert!(!matches!(state, State::Invalid));

          if matches!(state, State::Increasing) {
            state = State::Decreasing;
            prevs.push(for_after);

            cur_sum = a[i] as i64;
            for_before = 0;
            for_after = 0;
          } else if matches!(state, State::Decreasing) {
            cur_sum += a[i] as i64;
            // No need to update for_before and for_after
          }
        }
      }

      return max_sum;
    }
}
