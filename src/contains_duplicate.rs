use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut counts: HashSet<i32> = HashSet::new();

        for item in nums {
            if counts.contains(&item) {
                return true;
            } else {
                counts.insert(item);
            }
        }

        false
    }

    pub fn contains_duplicate_hashset_length(nums: Vec<i32>) -> bool {
        let counts: HashSet<i32> = nums.iter().cloned().collect();
        counts.len() != nums.len()
    }
}
