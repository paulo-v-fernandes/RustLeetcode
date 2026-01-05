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
            right: None
        }
    }
}

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        
        if root.is_none() {
            return true;
        }

        let root_node = root.unwrap();
        let p = root_node.borrow().left.clone();
        let q = root_node.borrow().right.clone();
        
        Self::is_same_mirror(p, q)
    }
    
    fn is_same_mirror(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() || q.is_none() {
            return false;
        }
        
        if let (Some(p_node), Some(q_node)) = (p, q) {
            let p_val = p_node.borrow().val;
            let q_val = q_node.borrow().val;

            if p_val != q_val {
                return false;
            }
            
            let left_same = Self::is_same_mirror(
                p_node.borrow().left.clone(),
                q_node.borrow().right.clone()
            );

            let right_same = Self::is_same_mirror(
                p_node.borrow().right.clone(),
                q_node.borrow().left.clone()
            );
            
            return left_same && right_same;
        }
        
        false
    }
}

fn main() {

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node_1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node_2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node_3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node_4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node_5 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node_6 = Rc::new(RefCell::new(TreeNode::new(3)));

    // Createe tree
    root.borrow_mut().left = Some(node_1.clone());
    root.borrow_mut().right = Some(node_2.clone());
    
    node_1.borrow_mut().left = Some(node_3);
    node_1.borrow_mut().right = Some(node_4);

    node_2.borrow_mut().left = Some(node_5);
    node_2.borrow_mut().right = Some(node_6);
    
    println!("Tree : {:?}", root);
    let result = Solution::is_symmetric(Some(root.clone()));

}