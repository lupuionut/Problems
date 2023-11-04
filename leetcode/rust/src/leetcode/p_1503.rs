// 1503. Last Moment Before All Ants Fall Out of a Plank
// -----------------------------------------------------

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let to_left = *left.iter().max().or(Some(&0)).unwrap();
        let to_right = *right.iter().min().or(Some(&n)).unwrap();

        to_left.max(n - to_right)
    }
}
