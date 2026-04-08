pub struct Solution;

fn main() {}

const MOD: i64 = 1_000_000_000 + 7;

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut nums = nums.into_iter().map(|it| it as i64).collect::<Vec<i64>>();

        for query in queries {
            let [li, ri, ki, vi] = query[..4] else { panic!() };
            let li = li as usize;
            let ri = ri as usize;
            let ki = ki as usize;
            let vi = vi as i64;

            let mut i = li;
            while i <= ri {
                nums[i] = (nums[i] * vi) % MOD;
                i = i + ki;
            }
        }

        let mut ans = nums[0];
        for &num in &nums[1..n] {
            ans = ans ^ num;
        }

        return ans as i32;
    }
}
