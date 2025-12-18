use leet_code::invert_binary_tree::Solution;
use utils::tree_node::TreeNode;

#[test]
// #[ignore]
fn case_1() {
    let result = Solution::invert_tree_n_fun(TreeNode::from_vec(vec![4, 2, 7, 1, 3, 6, 9]));
    assert_eq!(result, TreeNode::from_vec(vec![4, 7, 2, 9, 6, 3, 1]));
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(
        Solution::invert_tree(TreeNode::from_vec(vec![2, 1, 3])),
        TreeNode::from_vec(vec![2, 3, 1])
    );
}

#[test]
// #[ignore]
fn case_3() {
    assert_eq!(
        Solution::invert_tree(TreeNode::from_vec(vec![])),
        TreeNode::from_vec(vec![])
    );
}
