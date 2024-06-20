// 1552. Magnetic Force Between Two Balls
// --------------------------------------

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort();
        let mut left = 1;
        let mut right = *position.last().unwrap() - position[0];

        let mut best = left;
        while left <= right {
            let middle = (left + right) / 2;
            if Solution::is_good(middle, &position, m) {
                best = middle;
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }

        best
    }

    fn is_good(distance: i32, position: &Vec<i32>, total: i32) -> bool {
        let mut used = 0;
        let mut last = None;

        for i in 0..position.len() {
            if let Some(p) = last {
                if position[i] - p >= distance {
                    used += 1;
                    last = Some(position[i]);
                }
            } else {
                last = Some(position[i]);
                used += 1;
            }
        }
        used >= total
    }
}
