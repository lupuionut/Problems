// 1184. Distance Between Bus Stops

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len();
        let left: i32 = if destination < start {
            destination
        } else {
            start
        };

        let right: i32 = if destination < start {
            start
        } else {
            destination
        };

        let mut acc = 0;
        let mut prefix_sum = Vec::new();
        prefix_sum.push(0);
        for m in (0..n) {
            acc += distance[m];
            prefix_sum.push(acc);
        }

        let clockwise = prefix_sum[right as usize] - prefix_sum[left as usize];
        let counterclock =
            prefix_sum[n as usize] - prefix_sum[right as usize] + prefix_sum[left as usize];

        if clockwise < counterclock {
            clockwise
        } else {
            counterclock
        }
    }
}
