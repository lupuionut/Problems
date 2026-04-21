// 1722. Minimize Hamming Distance After Swap Operations
// -----------------------------------------------------
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct UnionFind {
    parent: Vec<i32>,
    components: i32,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for k in 0..size {
            parent[k] = k as i32;
        }

        UnionFind {
            parent,
            components: size as i32,
        }
    }

    fn find(&mut self, node: i32) -> i32 {
        if self.parent[node as usize] != node {
            self.parent[node as usize] = self.find(self.parent[node as usize]);
        }
        self.parent[node as usize]
    }

    fn union(&mut self, node1: i32, node2: i32) -> bool {
        let p1 = self.find(node1);
        let p2 = self.find(node2);

        if p1 == p2 {
            return false;
        }
        self.parent[p1 as usize] = p2;
        self.components -= 1;
        true
    }
}

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let mut ans = 0;
        let mut uf = UnionFind::new(100_001);
        let mut swaps: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for i in 0..allowed_swaps.len() {
            uf.union(allowed_swaps[i][0], allowed_swaps[i][1]);
        }
        for i in 0..source.len() {
            let parent = uf.find(i as i32);
            swaps
                .entry(parent)
                .and_modify(|mut h| {
                    h.entry(source[i]).and_modify(|v| *v += 1).or_insert(1);
                })
                .or_insert({
                    let mut h = HashMap::new();
                    h.insert(source[i], 1);
                    h
                });
        }

        for i in 0..source.len() {
            let parent = uf.find(i as i32);
            if let Some(replacements) = swaps.get_mut(&parent) {
                if let Some(&v) = replacements.get(&target[i]) {
                    if v > 0 {
                        replacements.insert(target[i], v - 1);
                    }
                }
            }
        }

        for (_, vals) in swaps.iter() {
            for (_, v) in vals.iter() {
                ans += v;
            }
        }

        ans
    }
}
