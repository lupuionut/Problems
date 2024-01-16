// 380. Insert Delete GetRandom O(1)
// ---------------------------------

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::collections::HashSet;

struct RandomizedSet {
    values: Vec<i32>,
    found: HashSet<i32>,
    keys: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            found: HashSet::new(),
            keys: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if !self.found.contains(&val) {
            self.found.insert(val);
            self.keys.insert(val, self.values.len());
            self.values.push(val);
            return true;
        }
        false
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.found.contains(&val) {
            let key = *self.keys.get(&val).unwrap();
            self.found.remove(&val);
            self.keys.remove(&val);
            let mut last = self.values.pop().unwrap();
            if last != val {
                self.values[key] = last;
                self.keys.insert(last, key);
            }
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let n = rng.sample(Uniform::new(0u64, self.values.len() as u64));
        self.values[n as usize]
    }
}
