// 3508. Implement Router
// ----------------------

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Router {
    // (destination, timestamp, source)
    keys: VecDeque<(i32, i32, i32)>,
    // (destination, timestamp, source)
    key_set: HashSet<(i32, i32, i32)>,
    // destination -> (timestamp, source)
    packets: HashMap<i32, VecDeque<(i32, i32)>>,
    limit: usize,
    length: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memoryLimit: i32) -> Self {
        let mut keys = VecDeque::new();
        let mut key_set = HashSet::new();
        let mut packets = HashMap::new();
        Router {
            keys,
            key_set,
            packets,
            limit: memoryLimit as usize,
            length: 0,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if self.key_set.contains(&(destination, timestamp, source)) {
            return false;
        }
        // check if full router
        if self.length == self.limit {
            self.forward_packet();
        }

        if let Some(mut entries) = self.packets.get_mut(&destination) {
            // add new
            entries.push_back((timestamp, source));
        } else {
            let mut entries = VecDeque::new();
            entries.push_back((timestamp, source));
            self.packets.insert(destination, entries);
        }
        self.keys.push_back((destination, timestamp, source));
        self.key_set.insert((destination, timestamp, source));
        self.length += 1;

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        // first out
        if let Some((d, t, s)) = self.keys.pop_front() {
            self.key_set.remove(&(d, t, s));
            self.packets.entry(d).and_modify(|es| {
                es.pop_front();
            });
            self.length -= 1;
            return vec![s, d, t];
        }
        vec![]
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let mut start = 0;
        let mut end = 0;

        if let Some(packets) = self.packets.get(&destination) {
            // search for start
            let mut l: usize = 0;
            let mut r: isize = packets.len() as isize - 1;

            while l as isize <= r {
                let mid = (l as isize + r) / 2;
                let mid = mid as usize;
                if packets[mid].0 < start_time {
                    l = mid + 1;
                } else {
                    r = mid as isize - 1;
                }
            }

            start = l;

            // search for end
            let mut l: usize = 0;
            let mut r: isize = packets.len() as isize - 1;

            while l as isize <= r {
                let mid = (l as isize + r) / 2;
                let mid = mid as usize;
                if packets[mid].0 <= end_time {
                    l = mid + 1;
                } else {
                    r = mid as isize - 1;
                }
            }
            end = l;
            if start >= packets.len() || packets[start].0 > end_time {
                return 0;
            }
        }
        (end - start) as i32
    }
}
