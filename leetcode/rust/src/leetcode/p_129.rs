// 129. Sum Root to Leaf Numbers
// -----------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn mk_sum(node: Option<Rc<RefCell<TreeNode>>>, curr: i32) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() {
                return curr + node.val;
            }
            return mk_sum(node.left.clone(), (curr + node.val) * 10)
                + mk_sum(node.right.clone(), (curr + node.val) * 10);
        }
        mk_sum(root, 0)
    }
}
