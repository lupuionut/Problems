// 1361. Validate Binary Tree Nodes
// --------------------------------

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut root_filter = vec![true; n as usize];

        for i in 0..n as usize {
            let left = left_child[i];
            let right = right_child[i];

            if left != -1 {
                root_filter[left as usize] = false;
            }
            if right != -1 {
                root_filter[right as usize] = false;
            }
        }

        let mut root = 0;
        let mut visited = vec![false; n as usize];

        let filtered = root_filter
            .iter()
            .enumerate()
            .filter(|(k, &v)| v == true)
            .map(|(k, v)| k)
            .collect::<Vec<_>>();

        if filtered.len() != 1 {
            return false;
        } else {
            root = filtered[0];
        }

        fn dfs(
            idx: usize,
            left_child: &Vec<i32>,
            right_child: &Vec<i32>,
            visited: &mut Vec<bool>,
        ) -> bool {
            if visited[idx] == true {
                return false;
            }

            visited[idx] = true;
            let l = left_child[idx];
            let r = right_child[idx];

            if l != -1 && r != -1 {
                return dfs(l as usize, left_child, right_child, visited)
                    && dfs(r as usize, left_child, right_child, visited);
            } else if l != -1 {
                return dfs(l as usize, left_child, right_child, visited);
            } else if r != -1 {
                return dfs(r as usize, left_child, right_child, visited);
            } else {
                return true;
            }
        }

        dfs(root, &left_child, &right_child, &mut visited) && visited.iter().all(|&x| x == true)
    }
}
