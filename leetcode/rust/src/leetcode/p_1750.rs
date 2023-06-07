// 1750. Minimum Length of String After Deleting Similar Ends
// ----------------------------------------------------------

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut left = 0;
        let mut right = s.len() - 1;
        let bytes = s.as_bytes();

        loop {
            if left >= right {
                break;
            }
            if bytes[left] != bytes[right] {
                break;
            }
            while (left + 1) < right && bytes[left + 1] == bytes[left] {
                left += 1;
            }
            while (left + 1) < right && bytes[right - 1] == bytes[right] {
                right -= 1;
            }
            if left == right {
                break;
            }
            left += 1;
            right -= 1;
        }

        if left == right {
            1
        } else {
            (right - left + 1) as i32
        }
    }
}
