use std::cell::RefCell;
use std::rc::Rc;
use utils::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            if let Some(node) = node {
                let right_depth = depth(node.borrow().right.clone(), diameter);
                let left_depth = depth(node.borrow().left.clone(), diameter);
                *diameter = (*diameter).max(left_depth + right_depth);
                return 1 + left_depth.max(right_depth);
            }
            0
        }

        let mut diameter = 0;
        depth(root, &mut diameter);
        diameter
    }
}
