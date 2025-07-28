use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn length_of_longest_substring_brute_force(s: String) -> i32 {
        let mut biggest: i32 = 0;
        for i in 0..s.len() {
            let mut letters: HashSet<char> = HashSet::new();
            for j in i..s.len() {
                if letters.contains(&s.chars().nth(j).unwrap()) {
                    break;
                }
                letters.insert(s.chars().nth(j).unwrap());
            }
            biggest = max(biggest, letters.len() as i32);
        }

        biggest
    }

    // Approach 2 - Sliding Window
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut biggest: i32 = 0;
        let mut letters: HashMap<char, i32> = HashMap::new();

        for (right, curr_char) in s.chars().enumerate() {
            if letters.contains_key(&curr_char) {
                left = max(left, letters[&curr_char] + 1);
            }
            letters.insert(curr_char, right as i32);
            biggest = max(biggest, right as i32 - left + 1);
        }

        biggest
    }
}
