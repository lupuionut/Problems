// 1386. Cinema Seat Allocation
// ----------------------------
use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut ans = 2 * n;
        let mut reserved: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..reserved_seats.len() {
            let r = reserved_seats[i][0];
            let c = reserved_seats[i][1];
            reserved.entry(r).and_modify(|cols| {cols.push(c)}).or_insert(vec![c]);
        }
        
        for (row, cols) in reserved.iter() {
            let mut s2 = true;
            let mut s4 = true;
            let mut s6 = true;
            for &col in cols {
                if col >= 2 && col < 4 {
                    s2 = false;
                }
                if col >= 4 && col < 6 {
                    s2 = false;
                    s4 = false;
                }
                if col >= 6 && col < 8 {
                    s4 = false;
                    s6 = false;
                }
                if col >= 8 && col < 10 {
                    s6 = false;
                }
            }
            if s2 == false {
                if s6 == false {
                    if s4 == true {
                        ans -= 1;
                    } else {
                        ans -= 2;
                    }
                } else {
                    ans -= 1;
                }
            } else {
                if s4 == false || s6 == false {
                    ans -= 1;
                }
            }
        }
        ans
    }
}
