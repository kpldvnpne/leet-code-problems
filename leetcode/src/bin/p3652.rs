pub struct Solution;

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
      let n = prices.len();
      let k = k as usize;
      let mut strategy_sums = vec![0; n];
      let mut price_sums = vec![0; n];
      let mut strategy_sum: i64 = 0;
      let mut price_sum: i64 = 0;
      for i in 0..n {
        strategy_sum += (prices[i] * strategy[i]) as i64;
        price_sum += prices[i] as i64;

        strategy_sums[i] = strategy_sum;
        price_sums[i] = price_sum;
      }

      let mut max_profit = strategy_sums[n-1] as i64;
      for i in 0..(n-k+1) {
        let before = if i > 0 { strategy_sums[i-1] } else {0};
        let for_second_half = price_sums[i+k-1] - price_sums[i+k/2-1];
        let after = strategy_sums[n-1] - strategy_sums[i+k-1];
        let cur_profit = before + for_second_half + after;
        max_profit = std::cmp::max(max_profit, cur_profit);

        // println!("{} {} {}", before, for_second_half, after);
      }

      return max_profit;
    }

    #[allow(dead_code)]
    fn max_profit_v2(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
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
