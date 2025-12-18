use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        while let Some(current) = queue.pop_front() {
            let current_borrowed = current.borrow();
            result.push(current_borrowed.val);

            if let Some(left) = &current_borrowed.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(right) = &current_borrowed.right {
                queue.push_back(Rc::clone(right));
            }
        }

        result
    }

    pub fn from_vec(vals: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        // 1. Cria a raiz da árvore
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0])));

        // 2. Fila para processar os nós por nível (Level-Order)
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < vals.len() {
            // Remove o próximo nó pai da fila para atribuir seus filhos
            if let Some(current) = queue.pop_front() {
                // Processa o filho à ESQUERDA
                if let Some(left_val) = vals.get(i) {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(*left_val)));
                    current.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                    i += 1;
                }

                // Processa o filho à DIREITA
                if let Some(right_val) = vals.get(i) {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(*right_val)));
                    current.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                    i += 1;
                }
            }
        }

        Some(root)
    }
}
