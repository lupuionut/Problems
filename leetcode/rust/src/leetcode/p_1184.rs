// 1184. Distance Between Bus Stops

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len();
        let mut d = 0;
        let mut ps = Vec::new();
        let mut clockwise = 0;
        let mut counterclock = 0;
        ps.push(0);

        for i in (0..n) {
            d += distance[i];
            ps.push(d);
        }

        if destination > start {
            clockwise = ps[destination as usize] - ps[start as usize];
            counterclock = ps[n as usize] - ps[destination as usize] + ps[start as usize];
        } else {
            clockwise = ps[start as usize] - ps[destination as usize];
            counterclock = ps[n as usize] - ps[start as usize] + ps[destination as usize];
        };

        if clockwise < counterclock {
            clockwise
        } else {
            counterclock
        }
    }
}
