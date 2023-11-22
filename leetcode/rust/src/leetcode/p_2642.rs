// 2642. Design Graph With Shortest Path Calculator
// ------------------------------------------------
// Dijkstra algo

use std::collections::BinaryHeap;
use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<(i32, i32)>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut adj = vec![vec![]; n as usize];
        edges.iter().for_each(|edge| {
            let from = edge[0];
            let to = edge[1];
            let edge_cost = edge[2];
            adj[from as usize].push((edge_cost, to));
        });
        Self { adj }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let from = edge[0];
        let to = edge[1];
        let edge_cost = edge[2];
        self.adj[from as usize].push((edge_cost, to));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        heap.push((0, node1));

        while heap.len() > 0 {
            if let Some((cost, node)) = heap.pop() {
                visited.insert(node);

                if node == node2 {
                    return -cost;
                }

                self.adj[node as usize]
                    .iter()
                    .for_each(|(next_cost, next_node)| {
                        if !visited.contains(next_node) {
                            heap.push((cost - next_cost, *next_node));
                        }
                    });
            }
        }

        -1
    }
}
