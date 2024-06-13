// 2037. Minimum Number of Moves to Seat Everyone
// ----------------------------------------------

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();
        let mut ans = 0;
        for i in 0..students.len() {
            ans += (students[i] - seats[i]).abs();
        }
        ans
    }
}
