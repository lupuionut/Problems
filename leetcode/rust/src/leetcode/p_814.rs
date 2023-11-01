// 814. Binary Tree Pruning
// ------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn is_ok(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if node.is_none() {
                return false;
            }
            let node = node.unwrap();
            if node.borrow().val == 1 {
                return true;
            } else {
                if is_ok(node.borrow().left.clone()) || is_ok(node.borrow().right.clone()) {
                    return true;
                }
            }
            false
        }

        fn dfs_check(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if is_ok(node.clone()) {
                let node = node.unwrap();
                let mut new_node = TreeNode::new(node.borrow().val);
                new_node.left = dfs_check(node.borrow().left.clone());
                new_node.right = dfs_check(node.borrow().right.clone());
                return Some(Rc::new(RefCell::new(new_node)));
            }
            None
        }

        dfs_check(root)
    }
}
