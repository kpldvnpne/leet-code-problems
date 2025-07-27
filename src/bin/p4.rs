// Uses mean sorted arrays

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 {
            return find_median(&nums2);
        }

        if nums2.len() == 0 {
            return find_median(&nums1);
        }

        if nums1.last() <= nums2.first() {
            return find_median_linear(&nums1, &nums2);
        }

        if nums2.last() <= nums1.first() {
            return find_median_linear(&nums2, &nums1);
        }

        // Now the complex part
        return find_median_binary(&nums1, &nums2);

    }

}

fn find_median_binary(arr1: &Vec<i32>, arr2: &Vec<i32>) -> f64 {
    // Use the bigger array to binary search on
    if arr2.len() > arr1.len() {
        return find_median_binary(arr2, arr1);
    }

    let total_size = arr1.len() + arr2.len();
    let median = total_size / 2;

    let mut left = 0;
    let mut right = arr1.len();

    loop {
        println!("{left}, {right}");

        if left == right {
            let mut candidates = vec![arr1[left], arr2[median - (left + 1)], arr1[left + 1], arr2[median - (left + 2)]];
            candidates.sort();
            return find_median(&candidates);
        }

        let mid1_idx = (left + right) / 2;
        println!("{mid1_idx}");
        let mid2_idx = median - (mid1_idx + 1);

        // Check if both are median
        if is_at_position(arr1[mid1_idx], arr2, mid2_idx) && is_at_position(arr2[mid2_idx], arr1, mid1_idx) {
            let is_even = total_size % 2 == 0;
            let a = arr1[mid1_idx];
            let b = arr2[mid2_idx];
            if is_even {
                return (a + b) as f64 / 2f64;
            } else {
                return max(a, b) as f64;
            }
        } else {
            if should_move_left(arr1[mid1_idx], arr2, mid2_idx) && should_move_right(arr1[mid1_idx], arr2, mid2_idx) {

            } else if should_move_left(arr1[mid1_idx], arr2, mid2_idx) {
                right = mid1_idx;
            } else {
                left = mid1_idx;
            }
        }
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn is_at_position(item: i32, arr: &Vec<i32>, idx: usize) -> bool {
    let last_idx = arr.len() - 1;
    return (idx == 0 || (idx > 0 && arr[idx - 1] <= item))
        && ((idx == last_idx) || (idx < last_idx && item <= arr[idx + 1]));
}

fn should_move_left(item: i32, arr: &Vec<i32>, idx: usize) -> bool {
    let last_idx = arr.len() - 1;
    return (idx == last_idx) || (idx < last_idx && item > arr[idx + 1]);
}

fn should_move_right(item: i32, arr: &Vec<i32>, idx: usize) -> bool {
    return (idx == 0) || (idx > 0 && arr[idx - 1] > item);
}

fn get_item(arr1: &Vec<i32>, arr2: &Vec<i32>, position: usize) -> f64 {
    if position < arr1.len() {
        return arr1[position] as f64;
    } else {
        return arr2[position - arr1.len()] as f64;
    }
}

fn find_median_linear(arr1: &Vec<i32>, arr2: &Vec<i32>) -> f64 {
    let total_size = arr1.len() + arr2.len();
    let is_even = total_size % 2 == 0;
    if is_even {
        let median_pos1 = total_size / 2 - 1;
        let median_pos2 = total_size / 2;

        return (get_item(arr1, arr2, median_pos1) + get_item(arr1, arr2, median_pos2)) as f64 / 2f64;
    } else {
        let median_pos = total_size / 2;
        return get_item(arr1, arr2, median_pos);
    }
}

fn find_median(nums: &Vec<i32>) -> f64 {
    let size = nums.len();
    let is_even = size % 2 == 0;
    if is_even {
        return (nums[size / 2 - 1] + nums[size / 2]) as f64 / 2.0f64;
    } else {
        return nums[size / 2] as f64;
    }
}

