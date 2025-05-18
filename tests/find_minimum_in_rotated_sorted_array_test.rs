use leet_code::find_minimum_in_rotated_sorted_array::Solution;

#[test]
fn case_1() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1)
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0)
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11)
}

#[test]
fn case_4() {
    assert_eq!(Solution::find_min(vec![2, 1]), 1)
}

#[test]
fn case_38() {
    assert_eq!(Solution::find_min(vec![3, 1, 2]), 1)
}
