//399. Evaluate Division

use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut adj: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        let mut ans: Vec<f64> = Vec::new();

        for i in 0..equations.len() {
            let u = equations[i][0].as_str();
            let v = equations[i][1].as_str();
            let cost = values[i];
            let pair = (v, cost);
            let mut default: Vec<(&str, f64)> = Vec::new();
            default.push(pair);
            adj.entry(u)
                .and_modify(|values| values.push(pair))
                .or_insert(default);
            let pair = (u, 1 as f64 / cost);
            let mut default: Vec<(&str, f64)> = Vec::new();
            default.push(pair);
            adj.entry(v)
                .and_modify(|values| values.push(pair))
                .or_insert(default);
        }

        fn bfs(start: &str, end: &str, adj: &HashMap<&str, Vec<(&str, f64)>>) -> f64 {
            if !adj.contains_key(start) || !adj.contains_key(end) {
                return -1 as f64;
            }
            let mut queue: Vec<(&str, f64)> = vec![];
            let mut visited: HashMap<&str, bool> = HashMap::new();
            queue.push((start, 1 as f64));
            visited.insert(start, true);

            while queue.len() > 0 {
                let (node, weight) = queue.pop().unwrap();
                let neighbours = adj.get(node);
                if node == end {
                    return weight;
                }

                if neighbours.is_some() {
                    for neighbour in neighbours.unwrap() {
                        if !visited.contains_key(neighbour.0) {
                            queue.push((neighbour.0, neighbour.1 * weight));
                            visited.insert(neighbour.0, true);
                        }
                    }
                }
            }

            return -1 as f64;
        }

        for i in 0..queries.len() {
            ans.push(bfs(&queries[i][0], &queries[i][1], &adj));
        }

        ans
    }
}
