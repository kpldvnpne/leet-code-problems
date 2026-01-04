struct Solution;

fn main() {
    println!("Sum divisors for 21 = {}", get_divisors_sum_if_4(21));
    println!("Sum divisors for 4 = {}", get_divisors_sum_if_4(4));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![21, 4, 7];
        let expected = 32;
        let actual = Solution::sum_four_divisors(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let nums = vec![21, 21];
        let expected = 64;
        let actual = Solution::sum_four_divisors(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = 0;
        let actual = Solution::sum_four_divisors(nums);
        assert_eq!(expected, actual);
    }
}

fn get_divisors_sum_if_4(num: i32) -> i32 {
    if num == 1 {
        return 0;
    }
    let mut total_divisor = 1 + num; // 1 and itself are its divisors
    let mut divisor = 2;  // start checking from 2
    let mut n_divisors = 2; // 1 and itself are its divisors
    while divisor * divisor <= num {
        if num % divisor == 0 {
            n_divisors += 1;
            total_divisor += divisor;
            let other_divisor = num / divisor;
            if other_divisor != divisor {
                n_divisors += 1;
                total_divisor += other_divisor;
            }

            if n_divisors > 4 { // Early exit
                return 0;
            }
        }
        divisor += 1;
    }

    if n_divisors != 4 { // If not equal to 4, then return 0 sum
        return 0;
    }

    return total_divisor;
}

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        for num in nums {
            total_sum += get_divisors_sum_if_4(num);
        }
        return total_sum;
    }
}