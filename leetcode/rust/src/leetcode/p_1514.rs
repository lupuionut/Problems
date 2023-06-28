// 1514. Path with Maximum Probability
// -----------------------------------
// use Dijkstra's algorithm with max probability

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut adj: Vec<Vec<(i32, f64)>> = vec![vec![]; n as usize];
        let mut heap: BinaryHeap<(String, i32)> = BinaryHeap::new();
        let mut visited: Vec<bool> = vec![false; n as usize];

        for i in 0..edges.len() {
            adj[edges[i][0] as usize].push((edges[i][1], succ_prob[i]));
            adj[edges[i][1] as usize].push((edges[i][0], succ_prob[i]));
        }

        heap.push((1.to_string(), start));
        visited[start as usize] = true;

        while heap.len() > 0 {
            let (prob, node) = heap.pop().unwrap();
            if node == end {
                return prob.parse::<f64>().unwrap();
            } else {
                visited[node as usize] = true;
                for (n, p) in &adj[node as usize] {
                    if visited[*n as usize] == false {
                        let prob = prob.parse::<f64>().unwrap();
                        let probability = (*p * prob).to_string();
                        heap.push((probability, *n));
                    }
                }
            }
        }

        0.0
    }
}
