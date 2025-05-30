// 2359. Find Closest Node to Given Two Nodes
// ------------------------------------------

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn calculate_cost(start: i32, edges: &Vec<i32>) -> Vec<i32> {
            let n = edges.len();
            let mut costs = vec![-1; n];
            let mut visited = vec![false; n];
            let mut q = vec![];
            q.push((start, 0));
            visited[start as usize] = true;
            while q.len() > 0 {
                if let Some((n, cost)) = q.pop() {
                    costs[n as usize] = cost;
                    let next = edges[n as usize];
                    if next != -1 && visited[next as usize] == false {
                        visited[next as usize] = true;
                        q.push((next, cost + 1));
                    }
                }
            }
            costs
        }

        let n1_cost = calculate_cost(node1, &edges);
        let n2_cost = calculate_cost(node2, &edges);

        let mut best_cost = i32::MAX;
        let mut idx = -1;
        for i in 0..edges.len() {
            if n1_cost[i] != -1 && n2_cost[i] != -1 {
                let cost = n1_cost[i].max(n2_cost[i]);
                if cost < best_cost {
                    best_cost = cost;
                    idx = i as i32;
                }
            }
        }

        if best_cost == i32::MAX {
            return -1;
        }
        idx
    }
}
