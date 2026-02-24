// 1022. Sum of Root To Leaf Binary Numbers
// ----------------------------------------
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        fn xsum(node: Option<Rc<RefCell<TreeNode>>>, acc: i32, sum: &mut i32) {
            let node = node.unwrap();
            let node = node.borrow();
            let acc = acc * 2;
            let acc = acc + node.val;
            if node.left.is_none() && node.right.is_none() {
                *sum += acc;
            }
            if node.left.is_some() {
                xsum(node.left.clone(), acc, sum);
            }
            if node.right.is_some() {
                xsum(node.right.clone(), acc, sum);
            }
        }
        xsum(root, 0, &mut ans);
        ans
    }
}
