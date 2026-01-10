struct Solution;

fn main() {}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let expected = 6;
        let actual = Solution::trap(height);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let expected = 9;
        let actual = Solution::trap(height);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test6() {
        let height = vec![4, 2, 3];
        let expected = 1;
        let actual = Solution::trap(height);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test320() {
        let height = vec![0, 2, 8, 2, 9, 7, 4];
        let expected = 6;
        let actual = Solution::trap(height);
        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_i = 0;
        let mut right_i = height.len() - 1;
        let mut left_max = height[left_i];
        let mut right_max = height[right_i];

        let mut water = 0;
        while left_i <= right_i {
            while left_max <= right_max && left_i <= right_i {
                left_max = std::cmp::max(left_max, height[left_i]);
                water += left_max - height[left_i];
                // println!("Left {left_i}: {}", water);
                left_i += 1;
            }

            while right_max <= left_max && left_i <= right_i {
                right_max = std::cmp::max(right_max, height[right_i]);
                water += right_max - height[right_i];
                // println!("Right {right_i}: {}", water);

                // Prevent overflow
                if right_i == 0 {
                    left_i = 1;
                    break;
                }
                right_i -= 1;
            }
        }

        return water;

    }
}