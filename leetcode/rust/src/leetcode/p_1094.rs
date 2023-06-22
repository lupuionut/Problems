// 1094. Car Pooling
// -----------------

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut load = vec![0; 1001];

        for trip in trips {
            for i in trip[1] as usize..trip[2] as usize {
                load[i] += trip[0];
            }
        }

        for i in 0..load.len() {
            if load[i] > capacity {
                return false;
            }
        }

        true
    }
}
