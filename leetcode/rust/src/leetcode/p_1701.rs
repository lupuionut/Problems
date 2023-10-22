// 1701. Average Waiting Time
// --------------------------

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut curr_start: i64 = 0;
        let mut acc: i64 = 0;

        customers.iter().for_each(|intval| {
            let arrival = intval[0] as i64;
            let time = intval[1] as i64;

            if arrival > curr_start {
                curr_start = arrival;
            }
            let waiting = (curr_start + time - arrival);
            acc += waiting;
            curr_start = curr_start + time;
        });

        acc as f64 / customers.len() as f64
    }
}
