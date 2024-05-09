// 881. Boats to Save People
// -------------------------

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut high = people.len() as i32 - 1;
        let mut low = 0;
        let mut ans = 0;

        while low <= high {
            if (people[high as usize] + people[low as usize]) <= limit {
                high -= 1;
                low += 1;
            } else {
                high -= 1;
            }
            ans += 1;
        }

        ans
    }
}
