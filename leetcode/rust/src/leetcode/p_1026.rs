// 1026. Maximum Difference Between Node and Ancestor
// --------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans: Option<i32> = None;

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            c_min: Option<i32>,
            c_max: Option<i32>,
            ans: &mut Option<i32>,
        ) {
            if let Some(node) = node {
                let node = node.borrow();
                if c_min.is_none() {
                    dfs(node.left.clone(), Some(node.val), Some(node.val), ans);
                    dfs(node.right.clone(), Some(node.val), Some(node.val), ans);
                } else {
                    dfs(
                        node.left.clone(),
                        Some(node.val.min(c_min.unwrap())),
                        Some(node.val.max(c_max.unwrap())),
                        ans,
                    );
                    dfs(
                        node.right.clone(),
                        Some(node.val.min(c_min.unwrap())),
                        Some(node.val.max(c_max.unwrap())),
                        ans,
                    );
                }
            } else {
                let diff = c_max.unwrap() - c_min.unwrap();
                if let &mut Some(a) = ans {
                    *ans = Some(a.max(diff));
                } else {
                    *ans = Some(diff);
                }
            }
        }

        dfs(root, None, None, &mut ans);
        ans.unwrap()
    }
}
