// 437. Path Sum III
// -----------------

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut ans = 0;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, target: i64, sum: i64, ans: &mut i32) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let mut new_sum = sum + node.borrow().val as i64;
            if new_sum == target {
                *ans += 1;
            }
            dfs(node.borrow().left.clone(), target, new_sum, ans);
            dfs(node.borrow().right.clone(), target, new_sum, ans);
        }

        fn start_from_node(
            node: Option<Rc<RefCell<TreeNode>>>,
            target: i64,
            sum: i64,
            ans: &mut i32,
        ) {
            if node.is_none() {
                return;
            }
            dfs(node.clone(), target, 0, ans);
            let node = node.unwrap();
            start_from_node(node.borrow().left.clone(), target, 0, ans);
            start_from_node(node.borrow().right.clone(), target, 0, ans);
        }

        start_from_node(root, target_sum as i64, 0, &mut ans);
        ans
    }
}
