/*
    705. Design HashSet
    -------------------
*/

struct MyHashSet {
    values: Vec<i32>,
}

impl MyHashSet {
    fn new() -> Self {
        let values = vec![0; 1000001];
        Self { values }
    }

    fn add(&mut self, key: i32) {
        self.values[key as usize] = 1;
    }

    fn remove(&mut self, key: i32) {
        self.values[key as usize] = 0;
    }

    fn contains(&self, key: i32) -> bool {
        if self.values[key as usize] == 1 {
            return true;
        }
        false
    }
}
