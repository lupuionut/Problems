// 1261. Find Elements in a Contaminated Binary Tree
// -------------------------------------------------

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

struct FindElements {
    values: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = HashSet::new();
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, val: i32, values: &mut HashSet<i32>) {
            if node.is_none() {
                return;
            }
            values.insert(val);
            let node = node.unwrap();
            let node = node.borrow();

            if node.left.is_some() {
                dfs(node.left.clone(), val * 2 + 1, values);
            }

            if node.right.is_some() {
                dfs(node.right.clone(), val * 2 + 2, values);
            }
        }
        dfs(root, 0, &mut values);
        FindElements { values }
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}
