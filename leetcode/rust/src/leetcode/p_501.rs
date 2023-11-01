// 501. Find Mode in Binary Search Tree
// ------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut current_val = None;
        let mut current_count = 0;
        let mut ans = vec![];
        let mut ans_count = 0;

        fn inorder(
            node: Option<Rc<RefCell<TreeNode>>>,
            current_val: &mut Option<i32>,
            current_count: &mut i32,
            ans: &mut Vec<i32>,
            ans_count: &mut i32,
        ) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();

            inorder(
                node.borrow().left.clone(),
                current_val,
                current_count,
                ans,
                ans_count,
            );

            if let &mut Some(val) = current_val {
                if val == node.borrow().val {
                    *current_count += 1;
                } else {
                    *current_val = Some(node.borrow().val);
                    *current_count = 1;
                }
            } else {
                *current_val = Some(node.borrow().val);
                *current_count = 1;
            }

            if current_count > ans_count {
                *ans = vec![current_val.unwrap()];
                *ans_count = *current_count;
            } else if current_count == ans_count {
                ans.push(current_val.unwrap());
            }

            inorder(
                node.borrow().right.clone(),
                current_val,
                current_count,
                ans,
                ans_count,
            );
        }

        inorder(
            root,
            &mut current_val,
            &mut current_count,
            &mut ans,
            &mut ans_count,
        );
        ans
    }
}
