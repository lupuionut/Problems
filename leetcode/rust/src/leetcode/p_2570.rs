// 2570. Merge Two 2D Arrays by Summing Values
// -------------------------------------------

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() || j < nums2.len() {
            if i == nums1.len() {
                ans.push(nums2[j].clone());
                j += 1;
                continue;
            }
            if j == nums2.len() {
                ans.push(nums1[i].clone());
                i += 1;
                continue;
            }
            if nums1[i][0] < nums2[j][0] {
                ans.push(nums1[i].clone());
                i += 1;
            } else if nums1[i][0] > nums2[j][0] {
                ans.push(nums2[j].clone());
                j += 1;
            } else {
                ans.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            }
        }

        ans
    }
}
