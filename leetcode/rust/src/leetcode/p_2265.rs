// 2265. Count Nodes Equal to Average of Subtree
// ---------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
            if node.is_none() {
                return (0, 0);
            }

            let node = node.unwrap();
            let (left_sum, left_count) = dfs(node.borrow().left.clone(), ans);
            let (right_sum, right_count) = dfs(node.borrow().right.clone(), ans);
            let sum = left_sum + node.borrow().val + right_sum;
            let count = left_count + 1 + right_count;
            if sum / count == node.borrow().val {
                *ans += 1;
            }
            (sum, count)
        }

        let mut ans = 0;
        let _ = dfs(root, &mut ans);

        ans
    }
}
