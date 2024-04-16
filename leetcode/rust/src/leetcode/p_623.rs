// 623. Add One Row to Tree
// ------------------------

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn mk_node(
            val: i32,
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }

        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, curr_depth: i32) {
            if let Some(node) = node {
                if curr_depth + 1 == depth {
                    let mut node = node.borrow_mut();
                    node.left = mk_node(val, node.left.take(), None);
                    node.right = mk_node(val, None, node.right.take());
                } else {
                    traverse(&node.borrow().left, val, depth, curr_depth + 1);
                    traverse(&node.borrow().right, val, depth, curr_depth + 1);
                }
            }
        }

        if depth == 1 {
            mk_node(val, root, None)
        } else {
            traverse(&root, val, depth, 1);
            root
        }
    }
}
