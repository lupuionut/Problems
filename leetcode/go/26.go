package leetcode

/*
    26. Remove Duplicates from Sorted Array
    ---------------------------------------
    Use 2 vars left and right, and whenver right finds a value different than
    the current one, left + 1 = new value and continue to move right
*/

func removeDuplicates(nums []int) int {
    l, r := 0, 0
    max := len(nums)
    for {
        if r == max {
            return l + 1
        }
        if nums[l] != nums[r] {
            l = l+1
            nums[l] = nums[r]
        }
        r += 1
    }
}
