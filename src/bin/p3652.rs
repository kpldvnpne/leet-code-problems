struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let prices = vec![4, 2, 8];
    let strategy = vec![-1, 0, 1];
    let k = 2;

    let expected = 10;
    let actual = Solution::max_profit(prices, strategy, k);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let prices = vec![5, 4, 3];
    let strategy = vec![1, 1, 0];
    let k = 2;

    let expected = 9;
    let actual = Solution::max_profit(prices, strategy, k);
    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
      let mut original_profit: i64 = 0;
      let n = prices.len();
      for i in 0..n {
        original_profit += (prices[i] * strategy[i]) as i64;
      }
      let mut max_profit = original_profit;

      let k = k as usize;
      let mut cur_profit: i64 = 0;
      for i in 0..(k/2) {
        cur_profit += (prices[i] * 0) as i64;
      }
      for i in k/2..k {
        cur_profit += (prices[i] * 1) as i64;
      }

      for i in k..n {
        cur_profit += (prices[i] * strategy[i]) as i64;
      }

      max_profit = std::cmp::max(cur_profit, max_profit);
      for i in 1..(n-k+1) {
        // The previous: Remove hold and make it what the strategy was before
        cur_profit += (prices[i-1] * strategy[i-1]) as i64 - 0;

        // The middle: Remove previous selling and make it hold
        let mid = i + k / 2 - 1;
        cur_profit += 0 - (prices[mid] * 1) as i64;

        // The last: Remove previous value and make it selling
        let last = i + k - 1;
        cur_profit += (prices[last] * 1 - prices[last] * strategy[last]) as i64;

        max_profit = std::cmp::max(max_profit, cur_profit);
      }

      return max_profit;
    }
}
