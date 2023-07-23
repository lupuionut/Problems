// 894. All Possible Full Binary Trees
// -----------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        fn dfs(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            let mut ans: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

            if n % 2 == 0 {
                return ans;
            }
            if n == 1 {
                let node = TreeNode::new(0);
                ans.push(Some(Rc::new(RefCell::new(node))));
                return ans;
            }
            let mut i = 1;
            while i < n {
                let left = dfs(i);
                let right = dfs(n - i - 1);
                for l in &left {
                    for r in &right {
                        let mut root = TreeNode::new(0);
                        root.left = l.clone();
                        root.right = r.clone();
                        ans.push(Some(Rc::new(RefCell::new(root))));
                    }
                }

                i += 2;
            }
            ans
        }

        let ans = dfs(n);
        ans
    }
}
