use leet_code::median_of_two_sorted_arrays::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(vec![1, 3], vec![2]),
        2.0,
        "binary search version case 1"
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(vec![1, 3], vec![2]),
        2.0,
        "binary search version case 1"
    );
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(vec![1, 2], vec![3, 4]),
        2.5
    );
}

#[test]
// #[ignore]
fn case_2061() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(vec![1, 3], vec![2, 7]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(vec![1, 3], vec![2, 7]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(vec![1, 3], vec![2, 7]),
        2.5
    );
}

#[test]
// #[ignore]
fn case_2071() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(vec![1, 2], vec![1, 1]),
        1.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(vec![1, 2], vec![1, 1]),
        1.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(vec![1, 2], vec![1, 1]),
        1.0
    );
}

#[test]
// #[ignore]
fn case_2072() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(vec![1, 1], vec![1, 2]),
        1.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(vec![1, 1], vec![1, 2]),
        1.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(vec![1, 1], vec![1, 2]),
        1.0
    );
}

#[test]
// #[ignore]
fn case_2073() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4],
            vec![1, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4]
        ),
        3.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4],
            vec![1, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4]
        ),
        3.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4],
            vec![1, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4]
        ),
        3.0
    );
}

#[test]
// #[ignore]
fn case_special() {
    assert_eq!(
        Solution::find_median_sorted_arrays_merge_sort(
            vec![1, 1, 2, 3, 5, 8, 13, 21],
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        ),
        6.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_binary_search(
            vec![1, 1, 2, 3, 5, 8, 13, 21],
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        ),
        6.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_better_binary_search(
            vec![1, 1, 2, 3, 5, 8, 13, 21],
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        ),
        6.0
    );
}
