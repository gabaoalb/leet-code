use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn find_min_brute_force(mut nums: Vec<i32>) -> i32 {
        let mut right = 0;
        let mut changed = false;
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
    pub fn find_min(mut nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;

        while low <= high && high - low > 1 {
            let middle = low + (high - low) / 2;

            match nums[middle as usize].cmp(&nums[high]) {
                Ordering::Less => high = middle,
                Ordering::Equal => (),
                Ordering::Greater => low = middle,
            }
        }

        let rotation_value = nums.len() - high;
        nums.rotate_right(rotation_value);

        println!("low: {}", low);
        println!("high: {}", high);
        println!("nums: {:?}", nums);

        nums[0]
    }
}
