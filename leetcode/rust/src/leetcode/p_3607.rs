// 3607. Power Grid Maintenance
// ----------------------------
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parents = vec![0; c as usize + 1];
        let mut visited = vec![false; c as usize + 1];
        let mut adj = vec![vec![]; c as usize + 1];
        let mut groups = vec![];
        let mut group_parent = HashMap::new();
        let mut ans = vec![];
        let mut offline = HashSet::new();
        for i in 0..=c {
            parents[i as usize] = i as usize;
        }
        for c in connections {
            adj[c[0] as usize].push(c[1] as usize);
            adj[c[1] as usize].push(c[0] as usize);
        }
        for i in 1..adj.len() {
            if visited[i] == false {
                let mut h = BinaryHeap::new();
                let p = parents[i];
                visit(i, &mut visited, &adj, &mut h, &mut parents, p);
                group_parent.insert(parents[i], groups.len());
                groups.push(h);
            }
        }
        fn visit(
            i: usize,
            visited: &mut Vec<bool>,
            adj: &Vec<Vec<usize>>,
            h: &mut BinaryHeap<Reverse<i32>>,
            parents: &mut Vec<usize>,
            parent: usize,
        ) {
            if visited[i] == true {
                return;
            }
            h.push(Reverse(i as i32));
            visited[i] = true;
            parents[i] = parent;
            for &n in &adj[i] {
                if visited[n] == false {
                    visit(n, visited, adj, h, parents, parent);
                }
            }
        }

        for q in queries {
            let req = q[0];
            let station = q[1];
            if req == 2 {
                offline.insert(station);
            } else {
                let mut t = -1;
                if !offline.contains(&station) {
                    t = station;
                } else {
                    let parent = parents[station as usize];
                    if let Some(&k) = group_parent.get(&parent) {
                        loop {
                            if let Some(Reverse(c)) = groups[k as usize].pop() {
                                if !offline.contains(&c) {
                                    t = c;
                                    groups[k as usize].push(Reverse(c));
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                ans.push(t);
            }
        }

        ans
    }
}
