use leet_code::diameter_of_binary_tree::Solution;
use utils::tree_node::TreeNode;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::diameter_of_binary_tree(TreeNode::from_vec(vec![1, 2, 3, 4, 5])),
        3
    );
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(
        Solution::diameter_of_binary_tree(TreeNode::from_vec(vec![1, 2])),
        1
    );
}
