// 889. Construct Binary Tree from Preorder and Postorder Traversal
// ----------------------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut positions = vec![0; postorder.len() + 1];
        fn construct(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.len() == 0 {
                return None;
            }
            let head = preorder[0];
            let mut node = TreeNode::new(head);

            if preorder.len() > 1 {
                let next = preorder[1];
                let mut end = postorder.len();
                let mut size = 0;

                while size < end && postorder[size] != next {
                    size += 1;
                }
                size += 1;
                node.left = construct(&preorder[1..1 + size], &postorder[0..size - 1]);
                if size < end {
                    node.right = construct(&preorder[size + 1..], &postorder[size..end - 1]);
                }
            }

            Some(Rc::new(RefCell::new(node)))
        }

        construct(&preorder[0..], &postorder[0..])
    }
}
