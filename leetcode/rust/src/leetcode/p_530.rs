// 530. Minimum Absolute Difference in BST
// ---------------------------------------
// Inorder traverse the tree and compare the previous value with current one

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = 100_000;
        let mut previous: Option<i32> = None;

        fn inorder_traverse(
            node: Option<Rc<RefCell<TreeNode>>>,
            previous: &mut Option<i32>,
            min: &mut i32,
        ) {
            let n = node.unwrap();
            if n.borrow().left.is_some() {
                inorder_traverse(n.borrow().left.clone(), previous, min);
            }
            let val = n.borrow().val;
            if let Some(p) = previous {
                if val - *p < *min {
                    *min = val - *p;
                }
            }
            *previous = Some(val);
            if n.borrow().right.is_some() {
                inorder_traverse(n.borrow().right.clone(), previous, min);
            }
        }
        inorder_traverse(root, &mut previous, &mut min);
        min
    }
}
