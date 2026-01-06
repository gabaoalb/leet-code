pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let mut count = [0; 26];
        let (mut left, mut right) = (0, 0);
        let mut max_count = 0;
        let mut result = 0;

        while right < s_bytes.len() {
            let index = (s_bytes[right] - b'A') as usize;
            count[index] += 1;
            max_count = max_count.max(count[index]);

            // current window size is right - left + 1
            // by subtracting max_count from our window size,
            // we get the number of characters that need to be replaced in order to make all characters in the window the same
            // if that number is greater than k, we need to shrink the window from the left
            while (right - left + 1) as i32 - max_count > k {
                let left_index = (s_bytes[left] - b'A') as usize;
                count[left_index] -= 1;
                left += 1;
            }

            result = result.max((right - left + 1) as i32);
            right += 1;
        }

        result
    }
}
