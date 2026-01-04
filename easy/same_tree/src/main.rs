
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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
            

            let left_same = Self::is_same_tree(
                p_node.borrow().left.clone(),
                q_node.borrow().left.clone()
            );
            
            let right_same = Self::is_same_tree(
                p_node.borrow().right.clone(),
                q_node.borrow().right.clone()
            );
            
            return left_same && right_same;
        }
        
        false
    }
}

pub struct Solution;

fn main() {
    // Tree 1
    let root1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node1_1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node1_2 = Rc::new(RefCell::new(TreeNode::new(3)));
    root1.borrow_mut().right = Some(node1_2);
    root1.borrow_mut().left = Some(node1_1);
    // Tree 2 
    let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2_1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node2_2 = Rc::new(RefCell::new(TreeNode::new(3)));
    root2.borrow_mut().right = Some(node2_2);
    root2.borrow_mut().left = Some(node2_1);


    let result = Solution::is_same_tree(Some(root1.clone()), Some(root2.clone()));
    println!("p: {:?}", root1);
    println!("q: {:?}", root2);
    println!("Result: {}", result);
}
