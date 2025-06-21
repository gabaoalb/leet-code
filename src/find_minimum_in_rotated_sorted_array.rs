use std::cmp;

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn find_min_brute_force(mut nums: Vec<i32>) -> i32 {
        let (mut right, mut changed) = (0, false);

        for n in 0..(nums.len() - 1) {
            if nums[n] < nums[n + 1] && !changed {
                continue;
            } else {
                if !changed {
                    changed = true;
                }
                right += 1;
            }
        }

        nums.rotate_right(right);

        nums[0]
    }

    // Approach 2 - Binary Search
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, nums.len() - 1);
        let mut min = nums[low];

        while low <= high && high - low >= 1 {
            let middle = low + (high - low) / 2;

            min = cmp::min(min, nums[middle]);

            if nums[middle] >= nums[low] {
                low = middle + 1;
            } else {
                high = middle - 1;
            }

            min = cmp::min(min, nums[low]);
        }

        min
    }
}
