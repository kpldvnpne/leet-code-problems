// Better than 100% of the code out there.

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut set: HashMap<i32, usize> = HashMap::new();

        let first_element = nums[0];
        set.insert(first_element, 0);

        let remaining = target - first_element;

        for i in 1..nums.len() {
            let num = nums[i];
            if num == remaining {
                return vec![0, i as i32];
            }

            set.insert(num, i);
        }

        for i in 1..nums.len() {
            let num = nums[i];
            let remaining = target - num;

            if let Option::Some(remaining_index) = set.get(&remaining) {
                if i != *remaining_index {
                    return vec![i as i32, *remaining_index as i32];
                }
            }
        }

        return vec![];
    }
}
