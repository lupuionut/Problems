// 872. Leaf-Similar Trees
// -----------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leafs1: Vec<i32> = vec![];
        let mut leafs2: Vec<i32> = vec![];

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() {
                leafs.push(node.val);
                return;
            }

            dfs(node.left.clone(), leafs);
            dfs(node.right.clone(), leafs);
        }

        dfs(root1, &mut leafs1);
        dfs(root2, &mut leafs2);
        leafs1 == leafs2
    }
}
