// 2709. Greatest Common Divisor Traversal
// ---------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut uf = UnionFind::new(nums.len());
        let mut primes: HashMap<i32, Vec<usize>> = HashMap::new();
        let mx = *nums.iter().max().unwrap() + 1;
        let mut sieve = vec![0; mx as usize];
        for i in 2..mx as usize {
            if sieve[i] == 0 {
                let mut j = 2 * i;
                while j < mx as usize {
                    if sieve[j] == 0 {
                        sieve[j] = i as i32;
                    }
                    j += i;
                }
            }
        }

        nums.iter().enumerate().for_each(|(k, &v)| {
            let mut val = v;
            while val != 1 {
                let factor = if sieve[val as usize] != 0 {
                    sieve[val as usize]
                } else {
                    val
                };
                primes
                    .entry(factor)
                    .and_modify(|c| c.push(k))
                    .or_insert(vec![k]);
                while val % factor == 0 {
                    val /= factor;
                }
            }
        });

        for (k, v) in primes.iter() {
            for i in 0..v.len() - 1 {
                uf.union(v[i], v[i + 1]);
            }
        }

        for i in 1..nums.len() {
            if uf.find(i) != uf.find(i - 1) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
struct UnionFind {
    roots: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut roots = vec![0; size];
        for i in 0..size {
            roots[i] = i;
        }
        Self { roots }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.roots[x] != x {
            self.roots[x] = self.find(self.roots[x]);
        }
        self.roots[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x != parent_y {
            self.roots[parent_x] = parent_y;
        }
    }
}
