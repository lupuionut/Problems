// 606. Construct String from Binary Tree
// --------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut acc = vec![];
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<String>) {
            let n = node.unwrap();
            let val = n.borrow().val.to_string();
            acc.push(val);

            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                return;
            }

            if n.borrow().left.is_some() && n.borrow().right.is_some() {
                acc.push("(".to_string());
                dfs(n.borrow().left.clone(), acc);
                acc.push(")".to_string());
                acc.push("(".to_string());
                dfs(n.borrow().right.clone(), acc);
                acc.push(")".to_string());
            } else {
                if n.borrow().left.is_some() {
                    acc.push("(".to_string());
                    dfs(n.borrow().left.clone(), acc);
                    acc.push(")".to_string());
                }
                if n.borrow().right.is_some() {
                    acc.push("()(".to_string());
                    dfs(n.borrow().right.clone(), acc);
                    acc.push(")".to_string());
                }
            }
        }

        dfs(root, &mut acc);
        acc.iter().fold("".to_string(), |mut a, s| {
            a.push_str(s);
            a
        })
    }
}
