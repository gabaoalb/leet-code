use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut low, mut high) = (1, *piles.iter().max().unwrap());
        let mut min_speed = high;

        while low <= high {
            let middle = low + (high - low) / 2;

            let mut time: i64 = 0; // Alterado para i64
            for &bananas in piles.iter() {
                time += (bananas as i64 + middle as i64 - 1) / middle as i64;
            }

            match time.cmp(&(h as i64)) {
                Ordering::Greater => low = middle + 1,
                _ => {
                    min_speed = middle;
                    high = middle - 1;
                }
            }
        }

        min_speed
    }
}
