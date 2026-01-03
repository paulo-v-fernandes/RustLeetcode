
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::recursive(&root, &mut result);
        result
    }
    fn recursive(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>){

        if let Some(n) = node {
            Self::recursive(&n.borrow().left, result);
            
            result.push(n.borrow().val);
            Self::recursive(&n.borrow().right, result);
        }
    }
}

pub struct Solution;

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node_1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node_2 = Rc::new(RefCell::new(TreeNode::new(3)));
    // Incrementa o contador
    root.borrow_mut().right = Some(node_1.clone());
    node_1.borrow_mut().left = Some(node_2);

    println!("Árvore criada: {:?}", root);



    let result = Solution::inorder_traversal(Some(root));
    
    println!("Inorder traversal: {:?}", result);
    println!("Esperado: [1, 3, 2]");
}
