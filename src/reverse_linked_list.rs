use utils::listnode::{LinkedListToVec, ListNode, VecToLinkedList};

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        let vec = head.to_vector();
        let rev_vec: Vec<i32> = vec.into_iter().rev().collect();
        rev_vec.to_linked_list()
    }

    pub fn reverse_list_three_pointer(
        head: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        let mut prev: Option<Box<ListNode<i32>>> = None;
        let mut curr = head;

        while let Some(mut current_node) = curr {
            let next_temp = current_node.next.take();
            current_node.next = prev;
            prev = Some(current_node);
            curr = next_temp;
        }

        prev
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        fn helper(
            node: Option<Box<ListNode<i32>>>,
            prev: Option<Box<ListNode<i32>>>,
        ) -> Option<Box<ListNode<i32>>> {
            match node {
                None => prev,
                Some(mut current_node) => {
                    let next_temp = current_node.next.take();
                    current_node.next = prev;
                    helper(next_temp, Some(current_node))
                } // Some(mut current_node) => helper(
                  //     std::mem::replace(&mut current_node.next, prev), // posso usar um mem::replace aqui para trocar os valores
                  //     Some(current_node),
                  // ),
            }
        }

        helper(head, None)
    }
}
