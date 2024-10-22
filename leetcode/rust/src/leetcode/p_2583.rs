// 2583. Kth Largest Sum in a Binary Tree
// --------------------------------------

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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut levels = vec![];

        fn recurse(node: Option<Rc<RefCell<TreeNode>>>, i: usize, levels: &mut Vec<i64>) {
            if node.is_none() {
                return;
            }

            let n = node.unwrap();
            let n = n.borrow();

            if i >= levels.len() {
                levels.push(n.val as i64);
            } else {
                levels[i] += n.val as i64;
            }

            recurse(n.left.clone(), i + 1, levels);
            recurse(n.right.clone(), i + 1, levels);
        }

        recurse(root, 0, &mut levels);
        if levels.len() < k as usize {
            return -1;
        }
        let mut values = levels
            .into_iter()
            .enumerate()
            .map(|(k, v)| (v, k as i32))
            .collect::<Vec<_>>();
        values.sort_by(|a, b| b.0.cmp(&a.0));
        values[(k - 1) as usize].0
    }
}
