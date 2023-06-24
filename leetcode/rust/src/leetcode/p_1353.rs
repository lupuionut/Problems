// 1353. Maximum Number of Events That Can Be Attended
// ---------------------------------------------------

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        let mut occupied = SegmentTree::new(vec![1; 100001]);
        let mut max_ev = 0;

        events.sort_by(|a, b| {
            if a[1] == b[1] {
                a[0].cmp(&b[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        for event in events {
            if let Some(i) = occupied.search(
                1,
                0,
                occupied.size - 1,
                event[0] as usize - 1,
                event[1] as usize - 1,
            ) {
                occupied.update(i, 0);
                max_ev += 1;
            }
        }

        max_ev
    }
}

#[derive(Debug)]
struct SegmentTree {
    tree: Vec<i32>,
    size: usize,
}

impl SegmentTree {
    fn new(values: Vec<i32>) -> Self {
        fn make_power_of_two(n: i32) -> i32 {
            let mut n = n;
            let mut i = 0;
            if n & (n - 1) == 0 {
                return n;
            }
            while n != 0 {
                n = n >> 1;
                i += 1;
            }
            1 << i
        }

        let size = 2 * make_power_of_two(values.len() as i32) as usize;
        let mut tree = vec![1; size];
        let n = values.len();
        for i in 0..n {
            tree[n + i] = values[i];
        }
        for i in (1..n).rev() {
            tree[i] = tree[2 * i] | tree[2 * i + 1];
        }
        Self {
            tree,
            size: (size / 2) - 1,
        }
    }

    fn search(
        &self,
        node: usize,
        node_low: usize,
        node_high: usize,
        query_low: usize,
        query_high: usize,
    ) -> Option<usize> {
        if node_low > query_high || node_high < query_low {
            return None;
        }

        if self.tree[node] == 0 {
            return None;
        }

        if node_low == node_high {
            if self.tree[node] == 1 {
                return Some(node_low);
            } else {
                return None;
            }
        }

        let node_mid = (node_low + node_high) / 2;

        let left = self.search(2 * node, node_low, node_mid, query_low, query_high);
        if left.is_some() {
            return left;
        } else {
            return self.search(2 * node + 1, node_mid + 1, node_high, query_low, query_high);
        }
    }

    fn update(&mut self, position: usize, value: i32) {
        let position = (self.size + 1) + position;
        self.tree[position] = value;
        let mut i = position / 2;
        while i >= 1 {
            self.tree[i] = self.tree[2 * i] | self.tree[2 * i + 1];
            i /= 2;
        }
    }
}
