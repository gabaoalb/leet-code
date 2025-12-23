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
}
