// 1502. Can Make Arithmetic Progression From Sequence
// ---------------------------------------------------

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort();
        let delta = (arr[0] - arr[1]).abs();

        for i in 1..arr.len() {
            let current = (arr[i] - arr[i - 1]).abs();
            if current != delta {
                return false;
            }
        }

        true
    }
}
