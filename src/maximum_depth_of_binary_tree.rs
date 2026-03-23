use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use utils::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node_borrowed = node.borrow();
                1 + cmp::max(
                    Self::max_depth(node_borrowed.left.clone()),
                    Self::max_depth(node_borrowed.right.clone()),
                )
            }
            None => 0,
        }
    }
}
