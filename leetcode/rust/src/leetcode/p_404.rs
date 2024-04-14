// 404. Sum of Left Leaves
// -----------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn calc(node: Option<Rc<RefCell<TreeNode>>>, left: bool) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() && left == true {
                return node.val;
            }
            return calc(node.left.clone(), true) + calc(node.right.clone(), false);
        }

        calc(root, false)
    }
}
