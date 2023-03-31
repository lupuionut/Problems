package leetcode

/*
    27. Remove Element
    ------------------
*/

func removeElement(nums []int, val int) int {
    v := 0
    idx := []int{}

    for k := range nums {
        if nums[k] == val {
            idx = append(idx, k)
        } else {
            v += 1
            if len(idx) != 0 {
                nums[idx[0]] = nums[k]
                idx = append(idx, k)
                idx = idx[1:]
            }
        }
    }

    return v
}
