// 863. All Nodes Distance K in Binary Tree
// ----------------------------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut ans = vec![];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; 501];

        fn traverse(node: Option<Rc<RefCell<TreeNode>>>, graph: &mut Vec<Vec<i32>>) {
            if let Some(n) = node {
                let k = n.borrow().val as usize;
                if n.borrow().left.is_some() {
                    let m = n.borrow().left.as_ref().unwrap().borrow().val;
                    graph[k].push(m);
                    graph[m as usize].push(k as i32);
                    traverse(n.borrow().left.clone(), graph);
                }
                if n.borrow().right.is_some() {
                    let m = n.borrow().right.as_ref().unwrap().borrow().val;
                    graph[k].push(m);
                    graph[m as usize].push(k as i32);
                    traverse(n.borrow().right.clone(), graph);
                }
            }
        }

        traverse(root, &mut graph);
        let mut target = target.unwrap().borrow().val;
        let mut visited: Vec<bool> = vec![false; 501];
        let mut queue = VecDeque::new();
        queue.push_back((target, 0));

        while queue.len() > 0 {
            let (key, lvl) = queue.pop_front().unwrap();
            visited[key as usize] = true;
            if lvl == k {
                ans.push(key);
            } else {
                for adj in &graph[key as usize] {
                    if visited[*adj as usize] == false {
                        queue.push_back((*adj, lvl + 1));
                    }
                }
            }
        }

        ans
    }
}
