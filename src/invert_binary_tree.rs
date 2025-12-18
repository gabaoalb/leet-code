use std::cell::RefCell;
use std::rc::Rc;
use utils::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            {
                let mut borrowed = node.borrow_mut();
                let left = borrowed.left.take();
                let right = borrowed.right.take();
                borrowed.left = Solution::invert_tree(right);
                borrowed.right = Solution::invert_tree(left);
            }

            return Some(node);
        }

        None
    }

    // O que não funciona:
    pub fn invert_tree_n_fun(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let temp = node.borrow().left.clone();
            node.borrow_mut().left = node.borrow_mut().right.clone();
            node.borrow_mut().right = temp;

            Solution::invert_tree(node.borrow_mut().left.take());
            Solution::invert_tree(node.borrow_mut().right.take());

            return Some(node);
        }

        None
    }
}
