package leetcode

func SearchInsert(nums []int, target int) int {
    l := 0
    r := len(nums) - 1

    for true {
        if nums[l] == target {
            return l
        }
        if nums[r] == target {
            return r
        }

        if len(nums[l:r+1]) <= 2 {
            if target > nums[r] {
                return r + 1
            }
            if target < nums[l] {
                return l
            }
            if target > nums[l] {
                return l + 1
            }
        }

        half := (l + r)/2

        if target == nums[half] {
            return half
        } else {
            if target > nums[half] {
                l = half
            } else {
                r = half
            }
        }
    }
    return 0
}
