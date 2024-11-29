#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            let (val1, val2) = (l1.as_ref().unwrap().val, l2.as_ref().unwrap().val);
            if val1 <= val2 {
                current.next = Some(Box::new(ListNode::new(val1)));
                l1 = l1.unwrap().next;
            } else {
                current.next = Some(Box::new(ListNode::new(val2)));
                l2 = l2.unwrap().next;
            }
            current = current.next.as_mut().unwrap();
        }

        while l1.is_some() {
            current.next = Some(Box::new(ListNode::new(l1.as_ref().unwrap().val)));
            current = current.next.as_mut().unwrap();
            l1 = l1.unwrap().next;
        }

        while l2.is_some() {
            current.next = Some(Box::new(ListNode::new(l2.as_ref().unwrap().val)));
            current = current.next.as_mut().unwrap();
            l2 = l2.unwrap().next;
        }

        dummy_head.next
    }
}

fn main() {
    let mut list1 = Some(Box::new(ListNode::new(1)));
    list1.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    list1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut list2 = Some(Box::new(ListNode::new(1)));
    list2.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    list2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let merged_list = Solution::merge_two_lists(list1, list2);

    let mut current = merged_list;
    while let Some(node) = current {
        println!("{}", node.val);
        current = node.next;
    }
}
