pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![5, 2, 3, 1];
        let expected = 2;
        let actual = Solution::minimum_pair_removal(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 2];
        let expected = 0;
        let actual = Solution::minimum_pair_removal(nums);

        assert_eq!(expected, actual);
    }
}

fn is_sorted(nums: &Vec<i32>) -> bool {
    for i in 1..nums.len() {
        if nums[i-1] > nums[i] {
            return false;
        }
    }
    return true;
}

fn return_smallest_index(nums: &Vec<i32>) -> usize {
    let mut min_sum = nums[0] + nums[1];
    let mut min_sum_index = 0;
    for i in 2..nums.len() {
        if nums[i] + nums[i-1] < min_sum {
            min_sum = nums[i] + nums[i-1];
            min_sum_index = i-1;
        }
    }

    return min_sum_index;
}

fn replace_with_sum(nums: &Vec<i32>, pair_i: usize) -> Vec<i32> {
    let mut new_nums = Vec::with_capacity(nums.len() - 1);
    for i in 0..nums.len() {
        if i == pair_i {
            new_nums.push(nums[i] + nums[i+1]);
        } else if i == pair_i + 1 {
            continue;
        } else {
            new_nums.push(nums[i]);
        }
    }

    return new_nums;
}

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut n_ops = 0;
        let mut cur_nums = nums;
        while !is_sorted(&cur_nums) {
            cur_nums = replace_with_sum(&cur_nums, return_smallest_index(&cur_nums));
            n_ops += 1;
        }

        return n_ops;
    }
}