// 1743. Restore the Array From Adjacent Pairs
// -------------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited = HashSet::new();

        adjacent_pairs.iter().for_each(|pair| {
            let from = pair[0];
            let to = pair[1];
            graph
                .entry(from)
                .and_modify(|nodes| nodes.push(to))
                .or_insert(vec![to]);
            graph
                .entry(to)
                .and_modify(|nodes| nodes.push(from))
                .or_insert(vec![from]);
        });

        let start = graph
            .iter()
            .filter(|(k, v)| v.len() == 1)
            .map(|(k, v)| *k)
            .collect::<Vec<i32>>()[0];

        let mut current = Some(start);
        ans.push(start);
        visited.insert(start);

        while current.is_some() {
            let mut next = None;
            if let Some(nodes) = graph.get(&current.unwrap()) {
                if !visited.contains(&nodes[0]) {
                    next = Some(nodes[0]);
                    ans.push(nodes[0]);
                    visited.insert(nodes[0]);
                }
                if nodes.len() > 1 && !visited.contains(&nodes[1]) {
                    next = Some(nodes[1]);
                    ans.push(nodes[1]);
                    visited.insert(nodes[1]);
                }
            }

            current = next;
        }

        ans
    }
}
