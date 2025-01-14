// 3386. Button with Longest Push Time
// -----------------------------------

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut curr = vec![0, 0];
        let mut longest = (0, -1);
        for i in 0..events.len() {
            let delta = events[i][1] - curr[1];
            if delta > longest.1 {
                longest = (events[i][0], delta);
            }
            if delta == longest.1 && events[i][0] < longest.0 {
                longest = (events[i][0], delta);
            }
            curr = events[i].clone();
        }
        longest.0 as i32
    }
}
