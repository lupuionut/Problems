// 1578. Minimum Time to Make Rope Colorful
// ----------------------------------------
// Use a stack to keep only the highest value for a color

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut stack: Vec<(u8, i32)> = vec![];
        let colors = colors.as_bytes();
        let mut cost = 0;

        for i in 0..colors.len() {
            let entry = (colors[i], needed_time[i]);
            while stack.len() > 0 {
                let last = stack.last().unwrap();
                // different colors
                if last.0 != entry.0 {
                    stack.push(entry);
                    break;
                // same colors
                } else {
                    // if last has lower value, we don't want to keep, we add it to the cost
                    if last.1 < entry.1 {
                        cost += last.1;
                        stack.pop();
                    } else {
                        cost += entry.1;
                        break;
                    }
                }
            }
            if stack.len() == 0 {
                stack.push(entry);
            }
        }

        cost
    }
}
