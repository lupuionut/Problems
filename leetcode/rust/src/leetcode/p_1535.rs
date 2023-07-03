// 1535. Find the Winner of an Array Game
// --------------------------------------

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let stopper = *arr.iter().max().unwrap();
        let mut counter = 0;
        let mut best = arr[0];
        let mut i = 1;

        if best == stopper {
            return best;
        }

        loop {
            if counter == k {
                break;
            }
            if arr[i] == stopper {
                best = stopper;
                break;
            }

            if best > arr[i] {
                counter += 1;
            } else {
                counter = 1;
                best = arr[i];
            }
            i += 1;
        }

        best
    }
}
