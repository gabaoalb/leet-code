use leet_code::longest_repeating_character_replacement::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}
