// 110. Balanced Binary Tree
// -------------------------
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let node = node.borrow();
            let left = 1 + height(node.left.clone());
            let right = 1 + height(node.right.clone());
            left.max(right)
        }
        if root.is_none() {
            return true;
        }
        let node = root.unwrap();
        let node = node.borrow();
        let lh = height(node.left.clone());
        let rh = height(node.right.clone());
        if (lh - rh).abs() > 1 {
            return false;
        }
        Self::is_balanced(node.left.clone()) && Self::is_balanced(node.right.clone())
    }
}
