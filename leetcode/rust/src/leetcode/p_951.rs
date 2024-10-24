// 951. Flip Equivalent Binary Trees
// ---------------------------------

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if (root1.is_none() && root2.is_some()) || (root1.is_some() && root2.is_none()) {
            return false;
        }
        if root1.is_none() && root2.is_none() {
            return true;
        }
        let root1 = root1.unwrap();
        let root1 = root1.borrow();
        let root2 = root2.unwrap();
        let root2 = root2.borrow();

        if root1.val != root2.val {
            return false;
        }
        let flip = Self::flip_equiv(root1.left.clone(), root2.right.clone())
            && Self::flip_equiv(root1.right.clone(), root2.left.clone());
        let notflip = Self::flip_equiv(root1.left.clone(), root2.left.clone())
            && Self::flip_equiv(root1.right.clone(), root2.right.clone());
        flip || notflip
    }
}
