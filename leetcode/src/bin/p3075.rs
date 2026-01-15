struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        assert_eq!(4, Solution::maximum_happiness_sum(happiness, k));
    }
}

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_by(|a, b| b.cmp(a));

        let mut total_happiness: i64 = 0;
        let mut discount = 0;
        for i in 0..(k as usize) {
            total_happiness += std::cmp::max(happiness[i] - discount, 0) as i64;
            discount += 1;
        }

        return total_happiness;
    }
}
