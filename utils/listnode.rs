#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    pub fn append(&mut self, elem: T) {
        match self.next {
            Option::Some(ref mut next_address) => {
                next_address.append(elem);
            }
            Option::None => {
                let node = ListNode {
                    val: elem,
                    next: Option::None,
                };
                self.next = Option::Some(Box::new(node))
            }
        }
    }
}

pub trait LinkedListToVec<T> {
    fn to_vector(self) -> Vec<T>;
}

impl<T> LinkedListToVec<T> for Option<Box<ListNode<T>>>
where
    T: Copy,
{
    fn to_vector(self) -> Vec<T> {
        let mut curr = self.as_ref().unwrap().as_ref();
        let mut vector: Vec<T> = vec![];

        loop {
            vector.push(curr.val);
            match curr.next.as_ref() {
                Some(next) => curr = next.as_ref(),
                None => break,
            }
        }

        vector
    }
}

pub trait VecToLinkedList<T> {
    fn to_linked_list(self) -> Option<Box<ListNode<T>>>;
}

impl<T> VecToLinkedList<T> for Vec<T> {
    fn to_linked_list(self) -> Option<Box<ListNode<T>>> {
        let mut curr = None;
        for value in self.into_iter().rev() {
            let mut new_node = ListNode::new(value);
            new_node.next = curr;
            curr = Some(Box::new(new_node));
        }
        curr
    }
}
