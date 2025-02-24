// 2467. Most Profitable Path in a Tree
// ------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut bob_times: HashMap<i32, i32> = HashMap::new();
        let mut best = i32::MIN;

        edges.iter().for_each(|edge| {
            let u = edge[0];
            let v = edge[1];
            adj.entry(u).and_modify(|e| e.push(v)).or_insert(vec![v]);
            adj.entry(v).and_modify(|e| e.push(u)).or_insert(vec![u]);
        });

        fn dfs_bob(
            curr: i32,
            prev: i32,
            adj: &HashMap<i32, Vec<i32>>,
            time: i32,
            times: &mut HashMap<i32, i32>,
        ) -> bool {
            if curr == 0 {
                times.insert(0, time);
                return true;
            }
            if let Some(ns) = adj.get(&curr) {
                for &n in ns {
                    if n == prev {
                        continue;
                    }
                    if dfs_bob(n, curr, adj, time + 1, times) {
                        times.insert(curr, time);
                        return true;
                    }
                }
            }
            false
        }

        fn dfs_alice(
            curr: i32,
            prev: i32,
            adj: &HashMap<i32, Vec<i32>>,
            time: i32,
            bob_times: &HashMap<i32, i32>,
            profit: i32,
            best: &mut i32,
            amount: &Vec<i32>,
        ) {
            let mut profit = profit;
            if let Some(&t) = bob_times.get(&curr) {
                if t > time {
                    profit += amount[curr as usize];
                } else if t == time {
                    profit += (amount[curr as usize] / 2);
                }
            } else {
                profit += amount[curr as usize];
            }

            if let Some(ns) = adj.get(&curr) {
                if ns.len() == 1 && curr != 0 {
                    *best = *best.max(&mut profit);
                }
                for &n in ns {
                    if n == prev {
                        continue;
                    }
                    dfs_alice(n, curr, adj, time + 1, bob_times, profit, best, amount);
                }
            }
        }

        dfs_bob(bob, -1, &adj, 0, &mut bob_times);
        dfs_alice(0, -1, &adj, 0, &bob_times, 0, &mut best, &amount);

        best
    }
}
