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
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, path: String, ans: &mut Vec<String>) -> () {
            if node.is_none() {
                return;
            }
            if let Ok(node) = Rc::try_unwrap(node.unwrap()) {
                let n = node.into_inner();
                let mut newp = path;
                newp.push_str(n.val.to_string().as_str());
                if n.left.is_some() || n.right.is_some() {
                    newp.push_str("->");
                    dfs(n.right, newp.to_owned(), ans);
                    dfs(n.left, newp.to_owned(), ans);
                } else {
                    ans.push(newp.to_owned());
                }
            }
        }

        dfs(root, String::from(""), &mut ans);
        ans
    }
}
