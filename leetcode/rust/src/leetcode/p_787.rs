// 787. Cheapest Flights Within K Stops
// ------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj = vec![vec![]; (n + 1) as usize];
        flights.iter().for_each(|flight| {
            // src -> (dest, price)
            adj[flight[0] as usize].push((flight[1], flight[2]));
        });

        let mut heap = BinaryHeap::new();
        let mut visited = vec![-1; (n + 1) as usize];

        // (price, stops, node)
        heap.push(Reverse((0, 0, src)));
        while heap.len() > 0 {
            let Reverse((price, stops, node)) = heap.pop().unwrap();
            if node == dst {
                return price;
            }
            // if we have already a stop with better price and less stops
            if visited[node as usize] != -1 && visited[node as usize] <= stops {
                continue;
            }
            visited[node as usize] = stops;
            if stops > k || adj[node as usize].len() == 0 {
                continue;
            }
            for &(next_stop, next_price) in &adj[node as usize] {
                heap.push(Reverse((price + next_price, stops + 1, next_stop)));
            }
        }
        -1
    }
}
