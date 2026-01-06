pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let m = s2.len();

        if n == 0 {
            return true;
        }

        if n > m {
            return false;
        }

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let mut s1_count = [0i32; 26];
        let mut s2_count = [0i32; 26];

        for &b in s1_bytes {
            s1_count[(b - b'a') as usize] += 1;
        }

        for right in 0..m {
            let idx = (s2_bytes[right] - b'a') as usize;
            s2_count[idx] += 1;
            if right >= n {
                let left_idx = (s2_bytes[right - n] - b'a') as usize;
                s2_count[left_idx] -= 1;
            }
            if s1_count == s2_count {
                return true;
            }
        }

        false
    }
}
