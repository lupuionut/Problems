// 1305. All Elements in Two Binary Search Trees
// ---------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ans = Vec::new();

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = root {
                dfs(node.borrow().left.clone(), ans);
                ans.push(node.borrow().val);
                dfs(node.borrow().right.clone(), ans);
            }
        }

        dfs(root1, &mut ans);
        dfs(root2, &mut ans);
        ans.sort();

        ans
    }
}
