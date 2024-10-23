// 2641. Cousins in Binary Tree II
// -------------------------------

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut total_level_parent = HashMap::new();
        let mut total_level = vec![];

        fn recurse(
            node: Option<Rc<RefCell<TreeNode>>>,
            level: usize,
            total_level: &mut Vec<i64>,
            parent: i32,
            new_parent: i32,
            total_level_parent: &mut HashMap<(usize, i32), i64>,
        ) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let node = node.borrow();
            if level >= total_level.len() {
                total_level.push(node.val as i64);
            } else {
                total_level[level] += node.val as i64;
            }
            total_level_parent
                .entry((level, parent))
                .and_modify(|v| *v += node.val as i64)
                .or_insert(node.val as i64);
            recurse(
                node.left.clone(),
                level + 1,
                total_level,
                new_parent,
                new_parent * 2 + 1,
                total_level_parent,
            );
            recurse(
                node.right.clone(),
                level + 1,
                total_level,
                new_parent,
                new_parent * 2 + 2,
                total_level_parent,
            );
        }

        fn create_node(
            level: usize,
            parent: i32,
            new_parent: i32,
            node: Option<Rc<RefCell<TreeNode>>>,
            total_level: &Vec<i64>,
            total_level_parent: &HashMap<(usize, i32), i64>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if node.is_none() {
                return None;
            }
            let node = node.unwrap();
            let node = node.borrow();
            let val =
                (total_level[level] - total_level_parent.get(&(level, parent)).unwrap()) as i32;
            let mut new_node = TreeNode::new({ val });
            new_node.left = create_node(
                level + 1,
                new_parent,
                new_parent * 2 + 1,
                node.left.clone(),
                total_level,
                total_level_parent,
            );
            new_node.right = create_node(
                level + 1,
                new_parent,
                new_parent * 2 + 2,
                node.right.clone(),
                total_level,
                total_level_parent,
            );
            Some(Rc::new(RefCell::new(new_node)))
        }

        recurse(
            root.clone(),
            0,
            &mut total_level,
            -1,
            0,
            &mut total_level_parent,
        );
        create_node(0, -1, 0, root, &total_level, &total_level_parent)
    }
}
