// 706. Design HashMap
// -------------------

struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: vec![vec![]; 1000],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let h = (key % 1000) as usize;
        for i in 0..self.buckets[h].len() {
            let current = self.buckets[h][i];
            if current.0 == key {
                self.buckets[h][i].1 = value;
            }
        }
        self.buckets[h].push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        let h = (key % 1000) as usize;
        for i in 0..self.buckets[h].len() {
            let current = self.buckets[h][i];
            if current.0 == key {
                return current.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        self.put(key, -1);
    }
}
