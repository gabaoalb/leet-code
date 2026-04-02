use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use utils::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    // Recursive DFS solution
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

    // Iterative DFS solution
    pub fn max_depth_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut stack = vec![(root, 1)];
        let mut max_depth = 0;

        while let Some((node_option, depth)) = stack.pop() {
            if let Some(node) = node_option {
                max_depth = cmp::max(max_depth, depth);
                let node_borrowed = node.borrow();
                stack.push((node_borrowed.left.clone(), depth + 1));
                stack.push((node_borrowed.right.clone(), depth + 1));
            }
        }

        max_depth
    }

    // Interative BFS solution
    pub fn max_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        let mut depth = 0;

        while !queue.is_empty() {
            depth += 1;
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    let node_borrowed = node.borrow();
                    if node_borrowed.left.is_some() {
                        queue.push_back(node_borrowed.left.clone());
                    }
                    if node_borrowed.right.is_some() {
                        queue.push_back(node_borrowed.right.clone());
                    }
                }
            }
        }

        depth
    }
}
