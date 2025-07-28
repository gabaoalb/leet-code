use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn max_profit_brute_force(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        for left in 0..prices.len() {
            for right in left..prices.len() {
                max_profit = max(max_profit, prices[right] - prices[left]);
            }
        }
        max_profit
    }

    // Approach 2 - Two Pointers
    pub fn max_profit_two_pointer(prices: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 1);
        let mut max_profit = 0;

        while right < prices.len() {
            if prices[left] < prices[right] {
                max_profit = max(max_profit, prices[right] - prices[left]);
            } else {
                left = right;
            }
            right += 1;
        }

        max_profit
    }

    // Approach 3 - Dynamic Programming
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_val = prices[0];

        for curr_val in prices {
            min_val = min(min_val, curr_val);
            max_profit = max(max_profit, curr_val - min_val);
        }

        max_profit
    }
}
