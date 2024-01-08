// 938. Range Sum of BST
// ---------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, mut sum: &mut i32) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let node = node.borrow();
            if node.val >= low && node.val <= high {
                *sum += node.val;
            }
            dfs(node.left.clone(), low, high, sum);
            dfs(node.right.clone(), low, high, sum);
        }
        dfs(root, low, high, &mut sum);
        sum
    }
}
