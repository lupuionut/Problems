// 1710. Maximum Units on a Truck
// ------------------------------

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut ans = 0;
        let mut taken = 0;

        box_types.iter().for_each(|unit| {
            if taken < truck_size {
                let quantity = unit[0].min(truck_size - taken);
                ans += quantity * unit[1];
                taken += quantity;
            }
        });

        ans
    }
}
