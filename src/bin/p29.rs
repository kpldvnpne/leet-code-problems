pub struct Solution;

fn main() {}

fn get_positive_value(number: i32) -> u32 {
	if number < 0 {
		return (u32::MAX ^ number as u32) + 1;
	}

	return number as u32;
}

impl Solution {
	pub fn divide(dividend: i32, divisor: i32) -> i32 {
		if dividend == i32::MIN && divisor == -1 {
			// This will be beyond MAX, so return just the max
			return i32::MAX;
		}
		return Solution::divide_using_subtraction(dividend, divisor);
	}

	// This is not accepted
	fn divide_normally(dividend: i32, divisor: i32) -> i32 {
		return dividend / divisor;
	}

	fn divide_using_subtraction(dividend: i32, divisor: i32) -> i32 {
		let sign = if (dividend < 0 && divisor < 0) || (dividend >= 0) && (divisor >= 0) { 1 } else { -1 };

		let u32_dividend = get_positive_value(dividend);
		let u32_divisor = get_positive_value(divisor);

		return sign * Solution::divide_using_substraction_both_pos(u32_dividend, u32_divisor);
	}

	// This times out for dividend = 1 << 30 or 1 << 31 when divisor < 10
	fn divide_using_substraction_both_pos(dividend: u32, divisor: u32) -> i32 {
		let mut cur_dividend = dividend;
		let mut quotient = 0;
		while cur_dividend >= divisor {
			cur_dividend -= divisor;
			quotient += 1;
		}

		return quotient;
	}

	fn divide_bitwise(dividend: u32, divisor: u32) -> i32 {
		let mut dividend = dividend;
		let mut remainder = 0;
		let mut quotient = 0;

		for _ in 1..=32 {
			let first_bit = (dividend & (1 << 31)) >> 31;
			dividend = dividend << 1;
			remainder = (remainder << 1) + first_bit;

			if remainder >= divisor {
				remainder -= divisor;
				quotient = (quotient << 1) + 1;
			} else {
				quotient = quotient << 1;
			}
		}

		return quotient;
	}

	fn divide_using_bitwise_operator(dividend: i32, divisor: i32) -> i32 {
		let sign = if (dividend < 0 && divisor < 0) || (dividend >= 0) && (divisor >= 0) { 1 } else { -1 };

		let u32_dividend: u32 = get_positive_value(dividend);
		let u32_divisor = get_positive_value(divisor);

		return sign * Solution::divide_bitwise(u32_dividend, u32_divisor);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_works_for_positive_numbers() {
		assert!(Solution::divide(3, 2) == 1, "It is not true");
	}

	#[test]
	fn it_works_for_negative_numbers() {
		assert!(Solution::divide(-3, -2) == 1, "It is not true");
	}

	#[test]
	fn it_works_for_same_numbers() {
		assert!(Solution::divide(-3, -3) == 1);
	}

	#[test]
	fn it_works_for_neg_dividend_and_pos_divisor() {
		assert!(Solution::divide(-3, 2) == -1);
	}

	#[test]
	fn it_works_for_pos_dividend_and_neg_divisor() {
		assert!(Solution::divide(3, -2) == -1);
	}

	#[test]
	fn it_works_for_overflow() {
		Solution::divide(i32::MIN, -1);
	}

	#[test]
	fn it_works_for_zero() {
		Solution::divide(0, -1);
	}

	#[test]
	fn it_works_within_time_for_large_numers() {
		Solution::divide(2147483647, 1); // TODO: Takes too much time
	}

	#[test]
	fn it_doesnot_overflow_when_divisor_large() {
		println!("{}", Solution::divide(i32::MIN, i32::MIN));
		assert!(Solution::divide(i32::MIN, i32::MIN) == 1);
		assert!(Solution::divide(i32::MIN, i32::MAX) == -1);
		assert!(Solution::divide(i32::MAX, i32::MIN) == 0);
	}

	#[test]
	fn shit_and_return_bit() {
		let left_most_bit = 1 << 31;
		// -5 => 5 = 0000 0000 0000 0000 0000 0000 0000 0101 => 1111 1111 1111 1111 1111 1111 1111 1011
		// -1 => 5 = 0000 0000 0000 0000 0000 0000 0000 0001 => 1111 1111 1111 1111 1111 1111 1111 1111
		// Perform 2's complement and get u32, then divide
		println!("{}", left_most_bit);
	}

	#[test]
	fn get_positive_value_works() {
		assert!(get_positive_value(-1) == 1);
		assert!(get_positive_value(0) == 0);
		assert!(get_positive_value(i32::MAX) == i32::MAX as u32);
		assert!(get_positive_value(i32::MIN) == 2u32.pow(31));
	}

	#[test]
	fn divide_bitwise_works() {
		assert!(Solution::divide_bitwise(1, 1) == 1);
		assert!(Solution::divide_bitwise(2, 2) == 1);
		assert!(Solution::divide_bitwise(2, 3) == 0);
		assert!(Solution::divide_bitwise(3, 1) == 3);
		assert!(Solution::divide_bitwise(0, 1) == 0);
		assert!(Solution::divide_bitwise(25, 5) == 5);
		assert!(Solution::divide_bitwise(2u32.pow(1), 2u32.pow(1)) == 1);
		assert!(Solution::divide_bitwise(2u32.pow(31), 2u32.pow(31)) == 1);
	}
}
