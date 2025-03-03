// 2161. Partition Array According to Given Pivot
// ----------------------------------------------

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut smaller = 0;
        let mut equal = 0;
        let mut higher = 0;
        let mut ans = vec![-1; nums.len()];

        nums.iter().for_each(|&x| {
            if x < pivot {
                smaller += 1;
            } else if x > pivot {
                higher += 1;
            } else {
                equal += 1;
            }
        });

        let mut i = 0;
        let mut j = smaller as usize;

        while equal > 0 {
            ans[j] = pivot;
            j += 1;
            equal -= 1;
        }

        nums.iter().for_each(|&x| {
            if x < pivot {
                ans[i] = x;
                i += 1;
            } else if x > pivot {
                ans[j] = x;
                j += 1;
            }
        });

        ans
    }
}
