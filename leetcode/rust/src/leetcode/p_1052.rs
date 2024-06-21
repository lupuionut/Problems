// 1052. Grumpy Bookstore Owner
// ----------------------------

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut satisfied = 0;
        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                satisfied += customers[i];
            }
        }

        let mut best = 0;
        let mut window = 0;
        let mut left = 0;

        for right in 0..customers.len() {
            if grumpy[right] == 1 {
                window += customers[right];
            }
            if right >= left + minutes as usize {
                left = right - minutes as usize;
                if grumpy[left] == 1 {
                    window -= customers[left];
                }
            }
            best = best.max(satisfied + window);
        }

        best
    }
}
