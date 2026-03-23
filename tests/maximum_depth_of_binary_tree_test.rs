use leet_code::maximum_depth_of_binary_tree::Solution;
use utils::tree_node::TreeNode;

#[test]
fn case_1() {
    assert!(Solution::max_depth(TreeNode::from_vec(vec![3, 9, 20, 0, 0, 15, 7])) == 3);
}
