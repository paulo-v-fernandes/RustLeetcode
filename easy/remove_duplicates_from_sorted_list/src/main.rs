// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current {
            while let Some(next) = node.next.as_mut() {
                if node.val == next.val {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            current = node.next.as_mut();
        }

        head
    }
}

fn main() {
    let node3 = Box::new(ListNode::new(3));
    let node3_dup = Box::new(ListNode { val: 3, next: Some(node3) });
    let node2 = Box::new(ListNode { val: 2, next: Some(node3_dup) });
    let node1_dup = Box::new(ListNode { val: 1, next: Some(node2) });
    let head = Some(Box::new(ListNode { val: 1, next: Some(node1_dup) }));

    let result = Solution::delete_duplicates(head);

    println!("{:?}", result);
}
