// 671. Second Minimum Node In a Binary Tree
// -----------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut fmin: Option<i32> = None;
        let mut smin: Option<i32> = None;
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        queue.push(root);

        while queue.len() > 0 {
            let node = queue.pop().unwrap();
            let val = node.as_ref().unwrap().borrow().val;
            if fmin.is_none() {
                fmin = Some(val);
            } else {
                if fmin.unwrap() > val {
                    smin = fmin;
                    fmin = Some(val);
                } else {
                    if (smin.is_none() || smin.unwrap() > val) && fmin.unwrap() != val {
                        smin = Some(val);
                    }
                }
            }
            if node.as_ref().unwrap().borrow().left.is_some() {
                queue.push(node.as_ref().unwrap().borrow().left.clone());
                queue.push(node.as_ref().unwrap().borrow().right.clone());
            }
        }

        if smin.is_none() {
            -1
        } else {
            smin.unwrap()
        }
    }
}
