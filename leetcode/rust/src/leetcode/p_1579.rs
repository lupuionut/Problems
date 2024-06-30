// 1579. Remove Max Number of Edges to Keep Graph Fully Traversable
// ----------------------------------------------------------------

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
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut alice = UnionFind::new(n as usize);
        let mut bob = UnionFind::new(n as usize);
        let mut removed = 0;

        for i in 0..edges.len() {
            let edge = &edges[i];
            if edge[0] == 3 {
                let used_for_alice = alice.union(edge[1] - 1, edge[2] - 1);
                let used_for_bob = bob.union(edge[1] - 1, edge[2] - 1);
                if !used_for_alice && !used_for_bob {
                    removed += 1;
                }
            }
        }

        for i in 0..edges.len() {
            let edge = &edges[i];
            if edge[0] == 1 {
                let used_for_alice = alice.union(edge[1] - 1, edge[2] - 1);
                if !used_for_alice {
                    removed += 1;
                }
            } else if edge[0] == 2 {
                let used_for_bob = bob.union(edge[1] - 1, edge[2] - 1);
                if !used_for_bob {
                    removed += 1;
                }
            }
        }

        if alice.components != 1 || bob.components != 1 {
            return -1;
        }
        removed
    }
}
