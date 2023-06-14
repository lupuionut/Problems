// 530. Minimum Absolute Difference in BST
// ---------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values: Vec<i32> = vec![];
        let mut min = 100_000;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            let n = node.unwrap();
            values.push(n.borrow().val);
            if n.borrow().left.is_some() {
                dfs(n.borrow().left.clone(), values);
            }
            if n.borrow().right.is_some() {
                dfs(n.borrow().right.clone(), values);
            }
        }
        dfs(root, &mut values);
        values.sort();
        for i in 1..values.len() {
            if values[i] - values[i - 1] < min {
                min = values[i] - values[i - 1];
            }
        }

        min
    }
}
