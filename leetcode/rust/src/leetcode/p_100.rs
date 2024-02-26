// 100. Same Tree
// --------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(a), Some(b)) => {
                if a.borrow().val != b.borrow().val {
                    return false;
                }
                return Self::is_same_tree(a.borrow().left.clone(), b.borrow().left.clone())
                    && Self::is_same_tree(a.borrow().right.clone(), b.borrow().right.clone());
            }
        }
    }
}
