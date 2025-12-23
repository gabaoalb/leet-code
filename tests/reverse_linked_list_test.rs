use leet_code::reverse_linked_list::Solution;
use utils::listnode::VecToLinkedList;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::reverse_list_three_pointer(vec![1, 2, 3, 4, 5].to_linked_list()),
        vec![5, 4, 3, 2, 1].to_linked_list()
    );
}
