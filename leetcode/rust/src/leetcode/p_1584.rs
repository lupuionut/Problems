// 1584. Min Cost to Connect All Points
// ------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for k in 0..size {
            parent[k] = k as i32;
        }

        UnionFind { parent }
    }

    fn find(&mut self, node: i32) -> i32 {
        if self.parent[node as usize] != node {
            self.parent[node as usize] = self.find(self.parent[node as usize]);
        }
        self.parent[node as usize]
    }

    fn union(&mut self, node1: i32, node2: i32) {
        let p1 = self.find(node1);
        let p2 = self.find(node2);

        if p1 == p2 {
            return;
        }
        self.parent[p1 as usize] = p2;
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut uf = UnionFind::new(points.len());
        let mut heap = BinaryHeap::new();
        let mut n = 0;

        fn manhattan_distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
            (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
        }

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let distance = manhattan_distance(&points[i], &points[j]);
                heap.push(Reverse((distance, i as i32, j as i32)));
            }
        }

        while heap.len() > 0 {
            if let Some(Reverse((distance, p1, p2))) = heap.pop() {
                if uf.find(p1) != uf.find(p2) {
                    uf.union(p1, p2);
                    ans += distance;
                    n += 1;
                    if n == points.len() - 1 {
                        break;
                    }
                }
            }
        }

        ans
    }
}
