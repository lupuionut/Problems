package leetcode

/*
    260. Single Number III
    ----------------------
    Xoring all the numbers will find the xor of the two numbers.
    Create a mask that contains a bit present only in one of these
    two numbers.
*/

func singleNumber(nums []int) []int {
    xor := 0
    a, b := 0, 0

    for k := range nums {
        xor ^= nums[k]
    }
    mask := xor & -xor

    for k := range nums {
        if nums[k] & mask == mask {
            a ^= nums[k]
        } else {
            b ^= nums[k]
        }
    }

    return []int{a, b}
}
