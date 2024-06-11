// 1122. Relative Sort Array
// -------------------------

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut lookup = HashMap::new();
        arr2.iter().enumerate().for_each(|(k, &v)| {
            lookup.insert(v, k as i32);
        });

        arr1.sort_by(|a, b| {
            let pa = Self::get_position(a, &lookup);
            let pb = Self::get_position(b, &lookup);
            match (pa, pb) {
                (None, None) => a.cmp(&b),
                (None, _) => Ordering::Greater,
                (_, None) => Ordering::Less,
                (_, _) => pa.cmp(&pb),
            }
        });

        arr1
    }

    pub fn get_position(val: &i32, lookup: &HashMap<i32, i32>) -> Option<i32> {
        if let Some(&v) = lookup.get(val) {
            return Some(v);
        }
        None
    }
}
