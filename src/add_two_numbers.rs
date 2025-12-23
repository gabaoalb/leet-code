use utils::listnode::{LinkedListToVec, ListNode, VecToLinkedList};

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode<i32>>>,
        l2: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        let v1 = l1.to_vector();
        let v2 = l2.to_vector();

        let mut carry = 0;
        let mut vec_result: Vec<i32> = vec![];
        let mut i = 0;
        vec_result = loop {
            carry += v1.get(i).unwrap_or(&0) + v2.get(i).unwrap_or(&0);
            vec_result.push(carry % 10);
            carry /= 10;
            i += 1;
            if v1.get(i).is_none() && v2.get(i).is_none() && carry == 0 {
                break vec_result;
            }
        };

        vec_result.to_linked_list()
    }
}
