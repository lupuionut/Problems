// 2458. Height of Binary Tree After Subtree Removal Queries
// ---------------------------------------------------------

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut node_height = vec![-1; 100001];
        let mut route_height = vec![-1; 100001];
        let mut similar_node_height: HashMap<i32, Vec<i32>> = HashMap::new();

        fn visit(
            node: Option<Rc<RefCell<TreeNode>>>,
            level: i32,
            node_height: &mut Vec<i32>,
            route_height: &mut Vec<i32>,
            similar_node_height: &mut HashMap<i32, Vec<i32>>,
        ) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let node = node.borrow();
            let left_height = visit(
                node.left.clone(),
                level + 1,
                node_height,
                route_height,
                similar_node_height,
            );
            let right_height = visit(
                node.right.clone(),
                level + 1,
                node_height,
                route_height,
                similar_node_height,
            );
            node_height[node.val as usize] = level;
            similar_node_height
                .entry(level)
                .and_modify(|v| v.push(node.val))
                .or_insert(vec![node.val]);
            route_height[node.val as usize] = level + left_height.max(right_height);
            1 + left_height.max(right_height)
        }

        visit(
            root,
            0,
            &mut node_height,
            &mut route_height,
            &mut similar_node_height,
        );
        let mut ans = vec![];

        queries.iter().for_each(|&q| {
            let h = node_height[q as usize];
            if let Some(similar) = similar_node_height.get(&h) {
                let mut best = h - 1;
                similar.iter().for_each(|&node| {
                    if node != q {
                        let curr = route_height[node as usize];
                        best = best.max(curr);
                    }
                });
                ans.push(best);
            }
        });

        ans
    }
}
