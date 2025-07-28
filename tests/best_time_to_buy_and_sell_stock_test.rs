use leet_code::best_time_to_buy_and_sell_stock::Solution;

#[test]
fn case_1() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
