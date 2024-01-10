// 2385. Amount of Time for Binary Tree to Be Infected
// ---------------------------------------------------

use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut time = 0;
        let mut queue = VecDeque::new();
        let mut adj = vec![vec![]; 100_001];
        let mut visited = HashSet::new();

        fn build_graph(node: Option<Rc<RefCell<TreeNode>>>, adj: &mut Vec<Vec<i32>>) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let node = node.borrow();

            if node.left.is_some() {
                let child = node.left.clone().unwrap().borrow().val;
                adj[node.val as usize].push(child);
                adj[child as usize].push(node.val);
                build_graph(node.left.clone(), adj);
            }
            if node.right.is_some() {
                let child = node.right.clone().unwrap().borrow().val;
                adj[node.val as usize].push(child);
                adj[child as usize].push(node.val);
                build_graph(node.right.clone(), adj);
            }
        }
        build_graph(root, &mut adj);

        queue.push_back((start, 0));
        while queue.len() > 0 {
            let (curr_node, curr_time) = queue.pop_front().unwrap();
            visited.insert(curr_node);
            time = time.max(curr_time);
            for &neighbour in adj[curr_node as usize].iter() {
                if !visited.contains(&neighbour) {
                    queue.push_back((neighbour, curr_time + 1));
                }
            }
        }

        time
    }
}
