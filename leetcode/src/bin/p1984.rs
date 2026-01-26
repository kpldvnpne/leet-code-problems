pub struct Solution;

fn main() {}

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut min_diff = i32::MAX;
        for i in 0..(nums.len() - k as usize + 1) {
            let max = nums[i + k as usize - 1];
            let min = nums[i];
            let diff = max - min;

            min_diff = min_diff.min(diff);
        }

        return min_diff;
    }
}
