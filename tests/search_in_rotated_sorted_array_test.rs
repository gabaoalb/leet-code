use leet_code::search_in_rotated_sorted_array::Solution;

#[test]
fn case_1() {
    assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4)
}

#[test]
fn case_2() {
    assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1)
}

#[test]
fn case_3() {
    assert_eq!(Solution::search(vec![1], 0), -1)
}

#[test]
fn case_4() {
    assert_eq!(Solution::search(vec![1], 2), -1)
}

#[test]
fn case_7() {
    assert_eq!(Solution::search(vec![1, 3], 2), -1)
}

#[test]
fn case_9() {
    assert_eq!(Solution::search(vec![1, 3], 4), -1)
}

#[test]
fn case_132() {
    assert_eq!(Solution::search(vec![1, 3], 3), 1)
}

#[test]
fn case_139() {
    assert_eq!(Solution::search(vec![1, 3, 5], 1), 0)
}

#[test]
fn case_150() {
    assert_eq!(Solution::search(vec![1, 3, 5], 5), 2)
}
