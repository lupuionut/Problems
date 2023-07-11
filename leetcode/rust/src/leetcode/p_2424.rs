// 2424. Longest Uploaded Prefix
// -----------------------------
// Use a segment tree where each val is set to 0
// Whenever a video is uploaded, the val is set to 1
// Searching in the segment tree will give us the first position of 0
// that means that until that point we have only 1, so that should be
// the longest uploaded prefix.

struct LUPrefix {
    tree: SegmentTree,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        let mut tree = SegmentTree::new(0, (n + 1) as usize);
        tree.update(0, 1);
        Self { tree }
    }

    fn upload(&mut self, video: i32) {
        self.tree.update(video as usize, 1);
    }

    fn longest(&self) -> i32 {
        // search for the first occurance of 0 in the tree
        // whenever a video n is uploaded, the n key of tree is set to 1
        // if all videos are uploaded, the final result is None, meaning
        // that there is no 0 in the tree
        if let Some(next) = self.tree.search(1, 0, self.tree.size) {
            return next - 1;
        }
        self.tree.size as i32
    }
}

#[derive(Debug)]
struct SegmentTree {
    tree: Vec<i32>,
    size: usize,
}

impl SegmentTree {
    fn new(val: i32, len: usize) -> Self {
        fn make_power_of_two(n: usize) -> usize {
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

        let size = 2 * make_power_of_two(len);
        let mut tree = vec![val; size];
        for i in 0..len {
            tree[len + i] = tree[i];
        }
        for i in (1..len).rev() {
            tree[i] = tree[2 * i] & tree[2 * i + 1];
        }
        Self {
            tree,
            size: (size / 2) - 1,
        }
    }

    fn search(&self, node: usize, node_low: usize, node_high: usize) -> Option<i32> {
        if self.tree[node] == 1 {
            return None;
        }

        if node_low == node_high {
            if self.tree[node] == 0 {
                let key = node - self.size - 1;
                return Some(key as i32);
            } else {
                return None;
            }
        }

        let node_mid = (node_low + node_high) / 2;

        let left = self.search(2 * node, node_low, node_mid);
        if left.is_some() {
            return left;
        } else {
            return self.search(2 * node + 1, node_mid + 1, node_high);
        }
    }

    fn update(&mut self, position: usize, value: i32) {
        let position = (self.size + 1) + position;
        self.tree[position] = value;
        let mut i = position / 2;
        while i >= 1 {
            self.tree[i] = self.tree[2 * i] & self.tree[2 * i + 1];
            i /= 2;
        }
    }
}
