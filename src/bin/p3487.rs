pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut found_non_neg: bool = false;
        let mut max_negative: Option<i32> = Option::None;

        let mut sum: i32 = 0;
        let mut prev_nums = HashSet::new();

        for num in nums {
            if !found_non_neg {
                if num >= 0 { // num is non-negative
                    found_non_neg = true
                } else { // num is negative
                    if max_negative == Option::None {
                        max_negative = Option::Some(num);
                    } else {
                        max_negative = Option::Some(std::cmp::max(num, max_negative.unwrap()));
                    }
                }
            }
            if num > 0 && !prev_nums.contains(&num) {
                prev_nums.insert(num);
                sum += num
            }
        }

        if !found_non_neg {
            return max_negative.unwrap();
        }

        return sum;
    }
}
