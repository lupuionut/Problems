// 112. Path Sum
// -------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
            if node.is_none() {
                return false;
            }

            let node = node.unwrap();
            let new_sum = sum - node.borrow().val;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if new_sum == 0 {
                    return true;
                }
                return false;
            }

            let ans = dfs(node.borrow().left.clone(), new_sum)
                | dfs(node.borrow().right.clone(), new_sum);
            ans
        }

        dfs(root, target_sum)
    }
}
