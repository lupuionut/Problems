// 94. Binary Tree Inorder Traversal
// ---------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                dfs(n.left.clone(), ans);
                ans.push(n.val);
                dfs(n.right.clone(), ans);
            }
        }

        dfs(root, &mut ans);
        ans
    }
}
