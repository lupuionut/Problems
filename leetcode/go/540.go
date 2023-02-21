package leetcode

func SingleNonDuplicate(nums []int) int {
    size := len(nums)
    if size == 1 {
        return nums[0]
    }    
    for i := 0; i < size/2; {    
        if (nums[i] + nums[size - i - 1]) != (nums[i+1] + nums[size - i - 2]) {
            if nums[i] != nums[i+1] {
                return nums[i]
            } else {
                return nums[size - i - 1]
            }
        }
        i += 2
    }

    return nums[size/2]
}
