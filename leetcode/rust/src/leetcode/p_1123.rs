// 1123. Lowest Common Ancestor of Deepest Leaves
// ----------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut paths = vec![];
        let mut path = vec![];
        let mut max_level = -1;

        fn traverse(
            node: &Option<Rc<RefCell<TreeNode>>>,
            level: i32,
            max_level: &mut i32,
            path: &mut Vec<i32>,
            paths: &mut Vec<Vec<i32>>,
        ) {
            let n = node.as_ref().unwrap();
            let n = n.borrow();

            if n.left.is_none() && n.right.is_none() {
                path.push(n.val);
                if level > *max_level {
                    *max_level = level;
                    paths.clear();
                }
                if level == *max_level {
                    paths.push(path.clone());
                }
                path.pop();
                return;
            }
            path.push(n.val);
            if n.left.is_some() {
                traverse(&n.left, level + 1, max_level, path, paths);
            }
            if n.right.is_some() {
                traverse(&n.right, level + 1, max_level, path, paths);
            }
            path.pop();
        }

        fn find_node_with_val(
            node: &Option<Rc<RefCell<TreeNode>>>,
            val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let n = node.as_ref().unwrap();
            let n = n.borrow();

            if n.val == val {
                return node.clone();
            }
            if n.left.is_some() {
                let n = find_node_with_val(&n.left, val);
                if n.is_some() {
                    return n;
                }
            }
            if n.right.is_some() {
                let n = find_node_with_val(&n.right, val);
                if n.is_some() {
                    return n;
                }
            }
            None
        }

        traverse(&root, 0, &mut max_level, &mut path, &mut paths);
        let mut base = paths[0].clone();
        let mut n = base.len();
        for j in 1..paths.len() {
            let mut i = 0;
            while i < n && j < paths[j].len() {
                if base[i] != paths[j][i] {
                    n = n.min(i);
                    break;
                }
                i += 1;
            }
        }

        find_node_with_val(&root, base[n - 1])
    }
}
