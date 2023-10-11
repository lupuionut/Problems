// 2251. Number of Flowers in Full Bloom
// -------------------------------------
// Create a hashmap with key: time and value: a vector with
// all pairs of (type/val)
// type 0 is for flower event and value is +1 for blooming and -1 for decay.
// type 1 is for visit, and value is the index of visit in the people vec
// since 0 is for blooming, we make sure we count blooming events before any visit
//
// Iterating over the keys of the hashmap (time markers), we have a clear
// view of how many flowers are blooming at that marker. If that marker is
// for a visit, just set the blooming accumulator.

use std::collections::HashMap;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        // events : time -> [(type, val)]
        let mut events: HashMap<i32, Vec<(u8, i32)>> = HashMap::new();
        let mut ans = vec![-1; people.len()];

        flowers.iter().for_each(|intval| {
            let start = intval[0];
            let end = intval[1] + 1;

            // +1 blooming
            if let Some(marker) = events.get_mut(&start) {
                marker.push((0, 1));
            } else {
                events.insert(start, vec![(0, 1)]);
            }

            // -1 blooming
            if let Some(marker) = events.get_mut(&end) {
                marker.push((0, -1))
            } else {
                events.insert(end, vec![(0, -1)]);
            }
        });

        people.iter().enumerate().for_each(|(k, v)| {
            if let Some(marker) = events.get_mut(&v) {
                marker.push((1, k as i32))
            } else {
                events.insert(*v, vec![(1, k as i32)]);
            }
        });

        let mut acc = 0;
        let mut timekeys: Vec<&i32> = events.keys().collect();
        timekeys.sort();

        for &timekey in &timekeys {
            if let Some(marker) = events.get(timekey) {
                marker.iter().for_each(|&(event_type, v)| {
                    if event_type == 0 {
                        acc += v;
                    } else {
                        ans[v as usize] = acc;
                    }
                });
            }
        }

        ans
    }
}
