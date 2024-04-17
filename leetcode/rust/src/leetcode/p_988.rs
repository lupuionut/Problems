// 988. Smallest String Starting From Leaf
// ---------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, curr: &mut Vec<char>) -> String {
            let node = node.unwrap();
            let node = node.borrow();
            let chr = char::from_u32((97 + node.val) as u32).unwrap();
            curr.push(chr);

            let mut ans = "".to_string();

            if node.left.is_none() && node.right.is_none() {
                ans = curr.iter().rev().collect::<String>();
            }
            if node.left.is_some() && node.right.is_some() {
                let lft = dfs(node.left.clone(), curr);
                let rgt = dfs(node.right.clone(), curr);
                ans = lft.min(rgt);
            } else if node.left.is_some() {
                ans = dfs(node.left.clone(), curr);
            } else if node.right.is_some() {
                ans = dfs(node.right.clone(), curr);
            }

            curr.pop();
            ans
        }
        let mut curr = vec![];
        dfs(root, &mut curr)
    }
}
