// 1028. Recover a Tree From Preorder Traversal
// --------------------------------------------

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut deque = VecDeque::new();
        let mut dash = 0;
        let mut val = vec![];
        let mut chars = traversal.chars().collect::<Vec<char>>();
        let mut i = 0;

        while i < chars.len() {
            while i < chars.len() && chars[i] == '-' {
                dash += 1;
                i += 1;
            }
            while i < chars.len() && chars[i] != '-' {
                val.push(chars[i]);
                i += 1;
            }
            let v = val.iter().collect::<String>();
            let v = i32::from_str_radix(&v, 10).unwrap();
            deque.push_back((dash, v));
            dash = 0;
            val.clear();
        }

        fn construct(
            levels: &mut VecDeque<(i32, i32)>,
            level: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(&front) = levels.front() {
                if front.0 != level {
                    return None;
                }
                levels.pop_front();
                let mut node = TreeNode::new(front.1);
                node.left = construct(levels, level + 1);
                node.right = construct(levels, level + 1);
                Some(Rc::new(RefCell::new(node)))
            } else {
                return None;
            }
        }

        construct(&mut deque, 0)
    }
}
