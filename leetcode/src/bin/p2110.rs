pub struct Solution;

fn main() {}

fn sum_to_n(n: i64) -> i64 {
    n * (n+1) / 2
}

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut n_smooth = 0;
        let mut rolling = 1; // We start from 1 in the loop
        let n = prices.len();
        for i in 1..n {
            if prices[i-1] - prices[i] == 1 {
                rolling +=  1;
            } else {
                n_smooth += sum_to_n(rolling as i64);
                rolling = 1;
            }
        }

        n_smooth += sum_to_n(rolling);

        return n_smooth;
    }
}
