use leet_code::permutation_in_string::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
        true
    );
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
        false
    );
}
