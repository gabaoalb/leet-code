pub struct Solution;

impl Solution {
    // Approach 2 - Binary Search
    // Time Complexity: O(log n)
    // Space Complexity: O(1)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        let (mut low, mut high) = (0, nums.len() - 1);

        while low <= high {
            let middle = low + (high - low) / 2;

            if nums[middle] == target {
                return middle as i32;
            }

            if nums[low] <= nums[middle] {
                if target > nums[middle] || target < nums[low] {
                    low = middle + 1;
                } else if let Some(opt_subtract) = middle.checked_sub(1) {
                    high = opt_subtract;
                } else {
                    break;
                }
            } else {
                if target < nums[middle] || target > nums[high] {
                    if let Some(opt_subtract) = middle.checked_sub(1) {
                        high = opt_subtract;
                    } else {
                        break;
                    }
                } else {
                    low = middle + 1;
                }
            }
        }

        -1
    }
}
