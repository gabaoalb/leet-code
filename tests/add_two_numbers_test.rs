use leet_code::add_two_numbers::Solution;
use utils::listnode::{ListNode, VecToLinkedList};

#[test]
//#[ignore]
fn case_1() {
    let l1 = vec![2, 4, 3].to_linked_list();
    let l2 = vec![5, 6, 4].to_linked_list();

    assert_eq!(
        Solution::add_two_numbers(l1, l2),
        vec![7, 0, 8].to_linked_list()
    );
}

#[test]
// #[ignore]
fn case_2() {
    let l1 = vec![0].to_linked_list();
    let l2 = vec![0].to_linked_list();
    assert_eq!(Solution::add_two_numbers(l1, l2), vec![0].to_linked_list());
}

#[test]
// #[ignore]
fn case_3() {
    let l1 = vec![9, 9, 9, 9, 9, 9, 9].to_linked_list();
    let l2 = vec![9, 9, 9, 9].to_linked_list();
    assert_eq!(
        Solution::add_two_numbers(l1, l2),
        vec![8, 9, 9, 9, 0, 0, 0, 1].to_linked_list()
    );
}

#[test]
// #[ignore]
fn case_1566() {
    let l1 = vec![
        2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3,
        2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3,
        9,
    ]
    .to_linked_list();
    let l2 = vec![
        5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3,
        2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 9, 9, 9,
        9,
    ]
    .to_linked_list();
    assert_eq!(
        Solution::add_two_numbers(l1, l2),
        vec![
            7, 0, 8, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8,
            6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 1,
            4, 3, 9, 1
        ]
        .to_linked_list()
    );
}
