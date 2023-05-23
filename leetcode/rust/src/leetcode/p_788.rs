// 788. Rotated Digits
// -------------------

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut counter = 0;

        fn is_good(n: i32, state: bool) -> bool {
            let goods: Vec<i32> = [2, 5, 6, 9].to_vec();
            let nulls: Vec<i32> = [0, 1, 8].to_vec();

            let d = n % 10;

            for null in nulls {
                if d == null {
                    let next = n / 10;
                    if next != 0 {
                        return is_good(next, state);
                    }
                    return state;
                }
            }

            for good in goods {
                if d == good {
                    let next = n / 10;
                    if next != 0 {
                        return is_good(next, true);
                    }
                    return true;
                }
            }

            false
        }

        for i in 1..n + 1 {
            if is_good(i, false) {
                counter += 1;
            }
        }

        counter
    }
}
