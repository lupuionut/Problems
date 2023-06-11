// 1146. Snapshot Array
// --------------------
// Container to match (snap_id, index) -> value
// Updates to keep a list of all updates (snap ids) for a certain index
// In case the hashmap doesn't contain the (snap_id, index) key, do a
// binary search in the updates vec to find the corresponding last snap_id from
// where we can retrieve the corresponding value.

use std::collections::HashMap;

struct SnapshotArray {
    container: HashMap<(i32, i32), i32>,
    updates: Vec<Vec<i32>>,
    snap_id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut container: HashMap<(i32, i32), i32> = HashMap::new();
        let updates = vec![vec![]; length as usize];
        SnapshotArray {
            container,
            snap_id: 0,
            updates,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.updates[index as usize].push(self.snap_id);
        self.container.insert((self.snap_id, index), val);
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let mut key = (snap_id, index);
        if self.updates[index as usize].len() == 0 {
            return 0;
        } else {
            if let Some(&val) = self.container.get(&key) {
                return val;
            } else {
                match self.updates[index as usize].binary_search(&snap_id) {
                    Ok(id) => {
                        let key = (id as i32, index);
                        if let Some(&val) = self.container.get(&key) {
                            return val;
                        } else {
                            return 0;
                        }
                    }
                    Err(mut id) => {
                        if id == 0 {
                            return 0;
                        } else {
                            id -= 1;
                            let key = (self.updates[index as usize][id], index);
                            if let Some(&val) = self.container.get(&key) {
                                return val;
                            } else {
                                return 0;
                            }
                        }
                    }
                }
            }
        }
    }
}
