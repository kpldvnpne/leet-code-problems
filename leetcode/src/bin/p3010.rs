pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 12];
        let expected = 6;
        let actual = Solution::minimum_cost(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let nums = vec![5, 4, 3];
        let expected = 12;
        let actual = Solution::minimum_cost(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let nums = vec![10, 3, 1, 1];
        let expected = 12;
        let actual = Solution::minimum_cost(nums);

        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut min_2 = [i32::MAX; 2];
        for num in &nums[1..nums.len()] {
            let mut ins = *num;
            for i in 0..2 {
                if ins < min_2[i] {
                    let temp = min_2[i];
                    min_2[i] = ins;
                    ins = temp;
                }
            }
        }

        nums[0] + min_2.iter().sum::<i32>()
    }
}